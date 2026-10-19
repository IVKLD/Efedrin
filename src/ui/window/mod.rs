mod builder;
mod event_handler;
mod events;
mod filtering;
mod folder;
mod handlers;
mod loader;
mod playback;
mod scanner;
mod timers;

use crate::audio::AudioEngine;
use crate::config::AppConfig;
use crate::sound::{LibraryWatcher, SoundItem, SoundLibrary};
use crate::ui::controls::ControlsBar;
use crate::ui::sound_grid::SoundGrid;
use crate::ui::tag_bar::TagBar;
use events::LibraryEvent;
use gtk4::prelude::*;
use libadwaita as adw;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::time::Instant;

pub struct MainWindow {
    window: adw::ApplicationWindow,
    config: Rc<RefCell<AppConfig>>,
    audio_engine: AudioEngine,
    library: Rc<RefCell<SoundLibrary>>,
    sound_grid: Rc<SoundGrid>,
    tag_bar: Rc<TagBar>,
    status_label: gtk4::Label,
    count_label: gtk4::Label,
    controls: Rc<ControlsBar>,
    current_playing_id: Rc<RefCell<Option<String>>>,
    current_playing_item: Rc<RefCell<Option<SoundItem>>>,
    playback_start: Rc<Cell<Option<Instant>>>,
    playback_offset: Rc<Cell<f64>>,
    is_user_seeking: Rc<Cell<bool>>,
    load_generation: Arc<AtomicU64>,
    scan_sender: Sender<LibraryEvent>,
    watcher: Rc<RefCell<Option<LibraryWatcher>>>,
}

impl MainWindow {
    pub fn new(app: &adw::Application, audio_engine: AudioEngine) -> Rc<Self> {
        let config = Rc::new(RefCell::new(AppConfig::load()));
        let (v_m, v_l, v_mic, pt, mic) = {
            let c = config.borrow();
            (c.volume_master, c.volume_local, c.volume_mic, c.mic_passthrough, c.selected_microphone.clone())
        };

        let controls = Rc::new(ControlsBar::new(v_m, v_l, v_mic, pt, audio_engine.list_microphones(), mic.as_deref()));
        let sound_grid = Rc::new(SoundGrid::new());
        let tag_bar = Rc::new(TagBar::new());
        let w = builder::build_main_window(app, &controls, &tag_bar, &sound_grid);
        let (scan_sender, scan_rx) = channel::<LibraryEvent>();

        let main_window = Rc::new(Self {
            window: w.window, config, audio_engine, library: Rc::new(RefCell::new(SoundLibrary::new())),
            sound_grid, tag_bar, status_label: w.status_label, count_label: w.count_label, controls,
            current_playing_id: Rc::new(RefCell::new(None)), current_playing_item: Rc::new(RefCell::new(None)),
            playback_start: Rc::new(Cell::new(None)), playback_offset: Rc::new(Cell::new(0.0)),
            is_user_seeking: Rc::new(Cell::new(false)), load_generation: Arc::new(AtomicU64::new(0)),
            scan_sender, watcher: Rc::new(RefCell::new(None)),
        });

        timers::setup_unified_timer(&main_window, Rc::new(RefCell::new(scan_rx)));
        handlers::setup_window_handlers(&main_window, &w.header);

        if pt { let _ = main_window.audio_engine.set_mic_passthrough(true, mic.as_deref()); }
        let init_dir = main_window.config.borrow().sound_directory.clone();
        if let Some(dir) = init_dir { if dir.exists() { main_window.load_directory(&dir); } }
        main_window
    }

    pub fn present(&self) {
        self.window.present();
    }
}
