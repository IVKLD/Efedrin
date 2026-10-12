use crate::audio::AudioEngine;
use crate::config::AppConfig;
use crate::sound::{SoundItem, SoundLibrary};
use crate::ui::controls::ControlsBar;
use crate::ui::header::Header;
use crate::ui::sound_grid::SoundGrid;
use gtk4::prelude::*;
use gtk4 as gtk;
use libadwaita::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub struct MainWindow {
    window: adw::ApplicationWindow,
    config: Rc<RefCell<AppConfig>>,
    audio_engine: AudioEngine,
    library: Rc<RefCell<SoundLibrary>>,
    sound_grid: Rc<SoundGrid>,
    status_label: gtk::Label,
    count_label: gtk::Label,
    controls: Rc<ControlsBar>,
}

impl MainWindow {
    pub fn new(app: &adw::Application, audio_engine: AudioEngine) -> Rc<Self> {
        let config = Rc::new(RefCell::new(AppConfig::load()));
        let library = Rc::new(RefCell::new(SoundLibrary::new()));

        let cfg = config.borrow();
        let initial_vol_local = cfg.volume_local;
        let initial_vol_mic = cfg.volume_mic;
        let initial_passthrough = cfg.mic_passthrough;
        drop(cfg);

        // Build window
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

        let controls = Rc::new(ControlsBar::new(
            initial_vol_local,
            initial_vol_mic,
            initial_passthrough,
        ));
        let sound_grid = Rc::new(SoundGrid::new());

        let sep1 = gtk::Separator::new(gtk::Orientation::Horizontal);
        let sep2 = gtk::Separator::new(gtk::Orientation::Horizontal);

        // Bottom status bar
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

        let main_window = Rc::new(Self {
            window,
            config,
            audio_engine,
            library,
            sound_grid,
            status_label,
            count_label,
            controls,
        });

        main_window.setup_event_handlers(&header);

        // Restore mic passthrough state if enabled in config
        if initial_passthrough {
            let _ = main_window.audio_engine.set_mic_passthrough(true);
        }

        // Restore initial directory if configured
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

        // 1. Stop All button in header
        let this_stop = this.clone();
        header.stop_button().connect_clicked(move |_| {
            this_stop.stop_all();
        });

        // 2. Global Keyboard Shortcuts (Space or Escape to Stop All)
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

        // 3. Folder Picker Button
        let this_folder = this.clone();
        this.controls.folder_button().connect_clicked(move |_| {
            this_folder.open_folder_dialog();
        });

        // 4. Instant Search filtering
        let this_search = this.clone();
        this.controls.search_entry().connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            this_search.filter_sounds(&query);
        });

        // 5. Volume Slider Changes -> Save to Config
        let cfg_vol = this.config.clone();
        this.controls.vol_local_scale().connect_value_changed(move |scale| {
            cfg_vol.borrow_mut().volume_local = scale.value();
            let _ = cfg_vol.borrow().save();
        });

        let cfg_mic = this.config.clone();
        this.controls.vol_mic_scale().connect_value_changed(move |scale| {
            cfg_mic.borrow_mut().volume_mic = scale.value();
            let _ = cfg_mic.borrow().save();
        });

        // 6. Mic Passthrough Switch
        let engine_pt = this.audio_engine.clone();
        let cfg_pt = this.config.clone();
        let status_lbl_pt = this.status_label.clone();
        this.controls.passthrough_switch().connect_state_set(move |_, state| {
            let res = engine_pt.set_mic_passthrough(state);
            cfg_pt.borrow_mut().mic_passthrough = state;
            let _ = cfg_pt.borrow().save();

            if state {
                status_lbl_pt.set_label("Physical microphone mixed into virtual microphone");
            } else {
                status_lbl_pt.set_label("Physical microphone passthrough disabled");
            }

            if let Err(e) = res {
                eprintln!("Error toggling mic passthrough: {}", e);
            }
            glib::Propagation::Proceed
        });

        // 7. Window Close Request -> Graceful Shutdown & Cleanup
        let engine_shutdown = this.audio_engine.clone();
        let cfg_shutdown = this.config.clone();
        this.window.connect_close_request(move |_| {
            let _ = cfg_shutdown.borrow().save();
            engine_shutdown.shutdown();
            glib::Propagation::Proceed
        });
    }

    /// Opens native folder selection dialog
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

    /// Loads and displays audio files from the specified folder
    pub fn load_directory(self: &Rc<Self>, path: &Path) {
        let count = match self.library.borrow_mut().scan_directory(path) {
            Ok(c) => c,
            Err(e) => {
                self.status_label.set_label(&format!("Error scanning folder: {}", e));
                return;
            }
        };

        // Update config
        {
            let mut cfg = self.config.borrow_mut();
            cfg.sound_directory = Some(path.to_path_buf());
            let _ = cfg.save();
        }

        // Update folder button label
        let dir_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Sounds");
        self.controls.set_folder_label(dir_name);

        self.count_label.set_label(&format!("{} sounds loaded", count));
        self.status_label.set_label(&format!("Loaded folder: {}", path.display()));

        let items = self.library.borrow().items().to_vec();
        self.render_sound_cards(&items);
    }

    /// Filters and updates grid items based on search query
    fn filter_sounds(self: &Rc<Self>, query: &str) {
        let filtered = self.library.borrow().filter(query);
        self.render_sound_cards(&filtered);
    }

    /// Renders sound items into grid with play callbacks
    fn render_sound_cards(self: &Rc<Self>, items: &[SoundItem]) {
        let this = self.clone();
        let on_play = Rc::new(move |item: &SoundItem| {
            this.play_sound(item);
        });

        self.sound_grid.populate(items, on_play);
    }

    /// Plays sound using AudioEngine with current local and mic volume levels
    fn play_sound(self: &Rc<Self>, item: &SoundItem) {
        let vol_local = self.controls.local_volume();
        let vol_mic = self.controls.mic_volume();

        match self.audio_engine.play_sound(&item.path, vol_local, vol_mic) {
            Ok(_) => {
                self.status_label.set_label(&format!(
                    "Playing: {} (Local: {:.0}%, Mic: {:.0}%)",
                    item.title,
                    vol_local * 100.0,
                    vol_mic * 100.0
                ));
            }
            Err(e) => {
                self.status_label.set_label(&format!("Playback error: {}", e));
            }
        }
    }

    /// Stops all running sounds
    fn stop_all(self: &Rc<Self>) {
        self.audio_engine.stop_all();
        self.status_label.set_label("All playback stopped");
    }
}
