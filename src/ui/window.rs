use crate::audio::AudioEngine;
use crate::config::AppConfig;
use crate::sound::{SoundItem, SoundLibrary};
use crate::ui::controls::ControlsBar;
use crate::ui::header::Header;
use crate::ui::sound_grid::SoundGrid;
use gtk4 as gtk;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use std::cell::{Cell, RefCell};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};

enum LibraryEvent {
    ScanComplete {
        generation: u64,
        items: Vec<SoundItem>,
        folder_display: String,
    },
    DurationProbed {
        generation: u64,
        id: String,
        duration: f64,
    },
    ScanError {
        generation: u64,
        error: String,
    },
}

pub struct MainWindow {
    window: adw::ApplicationWindow,
    config: Rc<RefCell<AppConfig>>,
    audio_engine: AudioEngine,
    library: Rc<RefCell<SoundLibrary>>,
    sound_grid: Rc<SoundGrid>,
    status_label: gtk::Label,
    count_label: gtk::Label,
    controls: Rc<ControlsBar>,
    current_playing_id: Rc<RefCell<Option<String>>>,
    current_playing_item: Rc<RefCell<Option<SoundItem>>>,
    playback_start: Rc<Cell<Option<Instant>>>,
    playback_offset: Rc<Cell<f64>>,
    is_user_seeking: Rc<Cell<bool>>,
    load_generation: Arc<AtomicU64>,
    scan_sender: Sender<LibraryEvent>,
}

impl MainWindow {
    pub fn new(app: &adw::Application, audio_engine: AudioEngine) -> Rc<Self> {
        let config = Rc::new(RefCell::new(AppConfig::load()));
        let library = Rc::new(RefCell::new(SoundLibrary::new()));
        let current_playing_id = Rc::new(RefCell::new(None));
        let current_playing_item = Rc::new(RefCell::new(None));
        let playback_start = Rc::new(Cell::new(None));
        let playback_offset = Rc::new(Cell::new(0.0));
        let is_user_seeking = Rc::new(Cell::new(false));

        let cfg = config.borrow();
        let initial_vol_master = cfg.volume_master;
        let initial_vol_local = cfg.volume_local;
        let initial_vol_mic = cfg.volume_mic;
        let initial_passthrough = cfg.mic_passthrough;
        let initial_mic = cfg.selected_microphone.clone();
        drop(cfg);

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Efedrin Soundpad")
            .default_width(860)
            .default_height(620)
            .build();

        let header = Header::new();

        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        main_box.append(header.widget());

        let mics = audio_engine.list_microphones();
        let controls = Rc::new(ControlsBar::new(
            initial_vol_master,
            initial_vol_local,
            initial_vol_mic,
            initial_passthrough,
            mics,
            initial_mic.as_deref(),
        ));
        let sound_grid = Rc::new(SoundGrid::new());

        let sep1 = gtk::Separator::new(gtk::Orientation::Horizontal);
        let sep2 = gtk::Separator::new(gtk::Orientation::Horizontal);

        let status_bar = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(12)
            .margin_start(16)
            .margin_end(16)
            .margin_top(6)
            .margin_bottom(6)
            .build();

        let status_label = gtk::Label::builder()
            .label("Ready")
            .css_classes(["caption"])
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();

        let count_label = gtk::Label::builder()
            .label("0 sounds loaded")
            .css_classes(["caption", "dim-label"])
            .halign(gtk::Align::End)
            .build();

        status_bar.append(&status_label);
        status_bar.append(&count_label);

        main_box.append(controls.widget());
        main_box.append(&sep1);
        main_box.append(sound_grid.widget());
        main_box.append(&sep2);
        main_box.append(&status_bar);

        window.set_content(Some(&main_box));

        let load_generation = Arc::new(AtomicU64::new(0));
        let (scan_sender, scan_receiver) = channel::<LibraryEvent>();
        let scan_receiver = Rc::new(RefCell::new(scan_receiver));

        let main_window = Rc::new(Self {
            window,
            config,
            audio_engine,
            library,
            sound_grid,
            status_label,
            count_label,
            controls,
            current_playing_id,
            current_playing_item,
            playback_start,
            playback_offset,
            is_user_seeking,
            load_generation,
            scan_sender,
        });

        {
            let weak_window = Rc::downgrade(&main_window);
            let rx = scan_receiver.clone();
            glib::timeout_add_local(Duration::from_millis(25), move || {
                if let Some(this) = weak_window.upgrade() {
                    let mut processed = 0;
                    while let Ok(event) = rx.borrow().try_recv() {
                        this.handle_library_event(event);
                        processed += 1;
                        if processed >= 64 {
                            break;
                        }
                    }
                    glib::ControlFlow::Continue
                } else {
                    glib::ControlFlow::Break
                }
            });
        }

        {
            let engine_poll = main_window.audio_engine.clone();
            let current_id_poll = main_window.current_playing_id.clone();
            let current_item_poll = main_window.current_playing_item.clone();
            let grid_poll = main_window.sound_grid.clone();
            let status_lbl_poll = main_window.status_label.clone();
            let controls_poll = main_window.controls.clone();
            let start_poll = main_window.playback_start.clone();
            let offset_poll = main_window.playback_offset.clone();
            let seeking_poll = main_window.is_user_seeking.clone();

            let mut stopped_ticks = 0u8;
            glib::timeout_add_local(Duration::from_millis(100), move || {
                if current_id_poll.borrow().is_some() {
                    if !engine_poll.is_playing() {
                        stopped_ticks += 1;
                        if stopped_ticks >= 3 {
                            *current_id_poll.borrow_mut() = None;
                            *current_item_poll.borrow_mut() = None;
                            start_poll.set(None);
                            offset_poll.set(0.0);
                            grid_poll.set_active_sound(None);
                            status_lbl_poll.set_label("Ready");
                            controls_poll.reset_progress();
                            stopped_ticks = 0;
                        }
                    } else {
                        stopped_ticks = 0;
                        if !seeking_poll.get() {
                            if let (Some(started), Some(item)) =
                                (start_poll.get(), current_item_poll.borrow().as_ref())
                            {
                                let elapsed = offset_poll.get() + started.elapsed().as_secs_f64();
                                controls_poll.set_playback_progress(elapsed, item.duration_secs);
                            }
                        }
                    }
                } else {
                    stopped_ticks = 0;
                }
                glib::ControlFlow::Continue
            });
        }

        main_window.setup_event_handlers(&header);

        if initial_passthrough {
            let _ = main_window
                .audio_engine
                .set_mic_passthrough(true, initial_mic.as_deref());
        }

        let initial_dir = main_window.config.borrow().sound_directory.clone();
        if let Some(dir) = initial_dir {
            if dir.exists() {
                main_window.load_directory(&dir);
            }
        }

        main_window
    }

    pub fn present(&self) {
        self.window.present();
    }

    fn setup_event_handlers(self: &Rc<Self>, header: &Header) {
        let this = self.clone();

        let this_stop = this.clone();
        header.stop_button().connect_clicked(move |_| {
            this_stop.stop_all();
        });

        let key_controller = gtk::EventControllerKey::new();
        let this_key = this.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
            if keyval == gtk4::gdk::Key::Escape || keyval == gtk4::gdk::Key::space {
                this_key.stop_all();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        this.window.add_controller(key_controller);

        let this_folder = this.clone();
        this.controls.folder_button().connect_clicked(move |_| {
            this_folder.open_folder_dialog();
        });

        let this_search = this.clone();
        this.controls
            .search_entry()
            .connect_search_changed(move |entry| {
                let query = entry.text().to_string();
                this_search.filter_sounds(&query);
            });

        let cfg_master = this.config.clone();
        let engine_vol_master = this.audio_engine.clone();
        let controls_master = this.controls.clone();
        this.controls
            .vol_master_scale()
            .connect_value_changed(move |scale| {
                let master = scale.value();
                cfg_master.borrow_mut().volume_master = master;
                let _ = cfg_master.borrow().save();
                let local_eff = controls_master.local_volume() * master;
                let mic_eff = controls_master.mic_volume() * master;
                engine_vol_master.set_local_volume(local_eff);
                engine_vol_master.set_mic_volume(mic_eff);
            });

        let cfg_vol = this.config.clone();
        let engine_vol_local = this.audio_engine.clone();
        let controls_vol = this.controls.clone();
        this.controls
            .vol_local_scale()
            .connect_value_changed(move |scale| {
                let val = scale.value();
                cfg_vol.borrow_mut().volume_local = val;
                let _ = cfg_vol.borrow().save();
                let master = controls_vol.master_volume();
                engine_vol_local.set_local_volume(val * master);
            });

        let cfg_mic = this.config.clone();
        let engine_vol_mic = this.audio_engine.clone();
        let controls_mic = this.controls.clone();
        this.controls
            .vol_mic_scale()
            .connect_value_changed(move |scale| {
                let val = scale.value();
                cfg_mic.borrow_mut().volume_mic = val;
                let _ = cfg_mic.borrow().save();
                let master = controls_mic.master_volume();
                engine_vol_mic.set_mic_volume(val * master);
            });

        let engine_pt = this.audio_engine.clone();
        let cfg_pt = this.config.clone();
        let status_lbl_pt = this.status_label.clone();
        let controls_pt = this.controls.clone();
        this.controls
            .passthrough_switch()
            .connect_state_set(move |_, state| {
                let selected_mic = controls_pt.selected_microphone_node();
                let res = engine_pt.set_mic_passthrough(state, selected_mic.as_deref());
                cfg_pt.borrow_mut().mic_passthrough = state;
                let _ = cfg_pt.borrow().save();

                if state {
                    status_lbl_pt.set_label("Microphone passthrough enabled");
                } else {
                    status_lbl_pt.set_label("Microphone passthrough disabled");
                }

                if let Err(e) = res {
                    eprintln!("Error toggling mic passthrough: {}", e);
                }
                glib::Propagation::Proceed
            });

        let engine_mic_change = this.audio_engine.clone();
        let cfg_mic_change = this.config.clone();
        let controls_mic_change = this.controls.clone();
        this.controls
            .mic_drop_down()
            .connect_selected_notify(move |_| {
                let selected_node = controls_mic_change.selected_microphone_node();
                cfg_mic_change.borrow_mut().selected_microphone = selected_node.clone();
                let _ = cfg_mic_change.borrow().save();

                if controls_mic_change.passthrough_switch().is_active() {
                    let _ = engine_mic_change.set_mic_passthrough(true, selected_node.as_deref());
                }
            });

        let this_seek = this.clone();
        this.controls
            .progress_scale()
            .connect_change_value(move |_, _, target_val| {
                this_seek.seek_playback(target_val);
                glib::Propagation::Proceed
            });

        let engine_shutdown = this.audio_engine.clone();
        let cfg_shutdown = this.config.clone();
        this.window.connect_close_request(move |_| {
            let _ = cfg_shutdown.borrow().save();
            engine_shutdown.shutdown();
            glib::Propagation::Proceed
        });
    }

    fn open_folder_dialog(self: &Rc<Self>) {
        let dialog = gtk::FileChooserNative::builder()
            .title("Select Sounds Directory")
            .action(gtk::FileChooserAction::SelectFolder)
            .transient_for(&self.window)
            .modal(true)
            .accept_label("Open")
            .cancel_label("Cancel")
            .build();

        let this = self.clone();
        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        this.load_directory(&path);
                    }
                }
            }
        });

        dialog.show();
    }

    pub fn load_directory(self: &Rc<Self>, path: &Path) {
        let generation = self.load_generation.fetch_add(1, Ordering::SeqCst) + 1;

        {
            let mut cfg = self.config.borrow_mut();
            cfg.sound_directory = Some(path.to_path_buf());
            let _ = cfg.save();
        }

        let dir_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Sounds");
        self.controls.set_folder_label(dir_name);

        self.count_label.set_label("Scanning...");
        self.status_label
            .set_label(&format!("Scanning folder: {}...", path.display()));

        let path_buf = path.to_path_buf();
        let sender = self.scan_sender.clone();
        let active_gen = self.load_generation.clone();

        std::thread::spawn(move || {
            if generation != active_gen.load(Ordering::SeqCst) {
                return;
            }

            let items = match SoundLibrary::scan_path(&path_buf) {
                Ok(items) => items,
                Err(e) => {
                    let _ = sender.send(LibraryEvent::ScanError {
                        generation,
                        error: e.to_string(),
                    });
                    return;
                }
            };

            let folder_display = path_buf.display().to_string();
            let _ = sender.send(LibraryEvent::ScanComplete {
                generation,
                items: items.clone(),
                folder_display,
            });

            if items.is_empty() || generation != active_gen.load(Ordering::SeqCst) {
                return;
            }

            let work_items: Vec<(String, std::path::PathBuf)> = items
                .into_iter()
                .map(|item| (item.id, item.path))
                .collect();
            let queue = std::sync::Arc::new(std::sync::Mutex::new(work_items));

            let num_workers = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
                .clamp(2, 8);

            let mut handles = Vec::with_capacity(num_workers);
            for _ in 0..num_workers {
                let queue = queue.clone();
                let sender = sender.clone();
                let active_gen = active_gen.clone();

                handles.push(std::thread::spawn(move || {
                    loop {
                        if generation != active_gen.load(Ordering::SeqCst) {
                            break;
                        }

                        let next = {
                            let mut q = queue.lock().unwrap();
                            q.pop()
                        };

                        let Some((id, path)) = next else {
                            break;
                        };

                        let duration = SoundItem::probe_duration(&path);

                        if generation != active_gen.load(Ordering::SeqCst) {
                            break;
                        }

                        if duration > 0.0 {
                            let _ = sender.send(LibraryEvent::DurationProbed {
                                generation,
                                id,
                                duration,
                            });
                        }
                    }
                }));
            }

            for handle in handles {
                let _ = handle.join();
            }
        });
    }

    fn handle_library_event(self: &Rc<Self>, event: LibraryEvent) {
        match event {
            LibraryEvent::ScanComplete {
                generation,
                items,
                folder_display,
            } => {
                if generation != self.load_generation.load(Ordering::SeqCst) {
                    return;
                }
                let count = items.len();
                self.library.borrow_mut().set_items(items.clone());
                self.count_label
                    .set_label(&format!("{} sounds loaded", count));
                self.status_label
                    .set_label(&format!("Loaded folder: {}", folder_display));

                let search_text = self.controls.search_entry().text().to_string();
                if search_text.trim().is_empty() {
                    self.render_sound_cards(&items);
                } else {
                    self.filter_sounds(&search_text);
                }
            }
            LibraryEvent::DurationProbed {
                generation,
                id,
                duration,
            } => {
                if generation != self.load_generation.load(Ordering::SeqCst) {
                    return;
                }
                self.library.borrow_mut().update_item_duration(&id, duration);
                self.sound_grid.update_sound_duration(&id, duration);

                if self.current_playing_id.borrow().as_deref() == Some(&id) {
                    if let Some(item) = self.current_playing_item.borrow_mut().as_mut() {
                        item.update_duration(duration);
                    }
                }
            }
            LibraryEvent::ScanError { generation, error } => {
                if generation != self.load_generation.load(Ordering::SeqCst) {
                    return;
                }
                self.status_label
                    .set_label(&format!("Error scanning folder: {}", error));
                self.count_label.set_label("0 sounds loaded");
                self.render_sound_cards(&[]);
            }
        }
    }

    fn filter_sounds(self: &Rc<Self>, query: &str) {
        let filtered = self.library.borrow().filter(query);
        self.render_sound_cards(&filtered);
    }

    fn render_sound_cards(self: &Rc<Self>, items: &[SoundItem]) {
        let this = self.clone();
        let on_play = Rc::new(move |item: &SoundItem| {
            this.play_sound(item);
        });

        self.sound_grid
            .populate(items, on_play, self.current_playing_id.borrow().as_deref());
    }

    fn play_sound(self: &Rc<Self>, item: &SoundItem) {
        if self.current_playing_id.borrow().as_deref() == Some(&item.id) {
            self.stop_all();
            return;
        }

        let mut item = item.clone();
        if item.duration_secs <= 0.0 {
            let probed = SoundItem::probe_duration(&item.path);
            if probed > 0.0 {
                item.update_duration(probed);
                self.library.borrow_mut().update_item_duration(&item.id, probed);
                self.sound_grid.update_sound_duration(&item.id, probed);
            }
        }

        let master = self.controls.master_volume();
        let vol_local = self.controls.local_volume() * master;
        let vol_mic = self.controls.mic_volume() * master;

        match self
            .audio_engine
            .play_sound(&item.path, vol_local, vol_mic, 0.0)
        {
            Ok(_) => {
                *self.current_playing_id.borrow_mut() = Some(item.id.clone());
                *self.current_playing_item.borrow_mut() = Some(item.clone());
                self.playback_start.set(Some(Instant::now()));
                self.playback_offset.set(0.0);
                self.controls.set_playback_progress(0.0, item.duration_secs);
                self.sound_grid.set_active_sound(Some(&item.id));
                self.status_label.set_label(&format!(
                    "Playing: {} (Sound: {:.0}%, Local: {:.0}%, Mic: {:.0}%)",
                    item.title,
                    master * 100.0,
                    self.controls.local_volume() * 100.0,
                    self.controls.mic_volume() * 100.0
                ));
            }
            Err(e) => {
                *self.current_playing_id.borrow_mut() = None;
                *self.current_playing_item.borrow_mut() = None;
                self.playback_start.set(None);
                self.playback_offset.set(0.0);
                self.controls.reset_progress();
                self.sound_grid.set_active_sound(None);
                self.status_label
                    .set_label(&format!("Playback error: {}", e));
            }
        }
    }

    fn seek_playback(self: &Rc<Self>, target_secs: f64) {
        let current_item = self.current_playing_item.borrow().clone();
        if let Some(mut item) = current_item {
            if item.duration_secs <= 0.0 {
                let probed = SoundItem::probe_duration(&item.path);
                if probed > 0.0 {
                    item.update_duration(probed);
                    *self.current_playing_item.borrow_mut() = Some(item.clone());
                    self.library.borrow_mut().update_item_duration(&item.id, probed);
                    self.sound_grid.update_sound_duration(&item.id, probed);
                }
            }

            self.is_user_seeking.set(true);
            let master = self.controls.master_volume();
            let vol_local = self.controls.local_volume() * master;
            let vol_mic = self.controls.mic_volume() * master;
            let max_dur = if item.duration_secs > 0.0 {
                item.duration_secs
            } else {
                target_secs
            };
            let clamped = target_secs.clamp(0.0, max_dur);

            match self
                .audio_engine
                .play_sound(&item.path, vol_local, vol_mic, clamped)
            {
                Ok(_) => {
                    self.playback_offset.set(clamped);
                    self.playback_start.set(Some(Instant::now()));
                    self.controls
                        .set_playback_progress(clamped, item.duration_secs);
                }
                Err(e) => {
                    eprintln!("Failed to seek: {}", e);
                }
            }
            self.is_user_seeking.set(false);
        }
    }

    fn stop_all(self: &Rc<Self>) {
        self.audio_engine.stop_all();
        *self.current_playing_id.borrow_mut() = None;
        *self.current_playing_item.borrow_mut() = None;
        self.playback_start.set(None);
        self.playback_offset.set(0.0);
        self.controls.reset_progress();
        self.sound_grid.set_active_sound(None);
        self.status_label.set_label("All playback stopped");
    }
}
