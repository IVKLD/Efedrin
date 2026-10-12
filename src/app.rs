use crate::audio::AudioEngine;
use crate::error::Result;
use crate::ui::MainWindow;
use gtk4::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

pub const APP_ID: &str = "org.efedrin.Soundpad";

const CUSTOM_CSS: &str = "
.sound-card-button {
    padding: 6px;
    border-radius: 12px;
    transition: all 180ms ease-in-out;
}

.sound-card-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
}

.badge {
    padding: 2px 8px;
    border-radius: 8px;
    background-color: alpha(currentColor, 0.12);
    font-weight: bold;
}

.pill {
    padding: 4px 10px;
    border-radius: 14px;
    background-color: alpha(@accent_color, 0.15);
    color: @accent_color;
}
";

pub fn run() -> Result<()> {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    // 1. Startup: Load custom Adwaita CSS
    app.connect_startup(|_| {
        let provider = gtk::CssProvider::new();
        provider.load_from_data(CUSTOM_CSS);

        if let Some(display) = gtk4::gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    });

    // Audio engine is initialized once and retained across application lifetime
    let audio_engine_holder = Rc::new(RefCell::new(None));
    let engine_for_signal = audio_engine_holder.clone();

    // 2. Activate: Instantiate and display MainWindow
    let engine_slot = audio_engine_holder.clone();
    app.connect_activate(move |app| {
        let engine = match AudioEngine::new() {
            Ok(eng) => eng,
            Err(e) => {
                eprintln!("Warning: Failed to initialize PipeWire audio engine: {}", e);
                // Try to continue so UI still displays errors to user
                return;
            }
        };

        *engine_slot.borrow_mut() = Some(engine.clone());
        let window = MainWindow::new(app, engine);
        window.present();
    });

    // 3. Graceful termination on Unix signals (Ctrl+C / SIGTERM)
    glib::unix_signal_add_local(2, move || {
        println!("\nReceived SIGINT. Gracefully cleaning up PipeWire virtual devices...");
        if let Some(engine) = engine_for_signal.borrow().as_ref() {
            engine.shutdown();
        }
        std::process::exit(0);
    });

    app.run();
    Ok(())
}
