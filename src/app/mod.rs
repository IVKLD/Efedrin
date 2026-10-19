mod base_css;
mod css;
mod tag_css;

use crate::audio::AudioEngine;
use crate::error::Result;
use crate::ui::MainWindow;
use gtk4 as gtk;
use gtk4::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

pub const APP_ID: &str = "org.efedrin.Soundpad";

fn setup_style_and_icons() {
    let style_manager = adw::StyleManager::default();
    style_manager.set_color_scheme(adw::ColorScheme::ForceDark);

    let provider = gtk::CssProvider::new();
    provider.load_from_data(&css::load_app_css());

    if let Some(display) = gtk4::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let icon_theme = gtk::IconTheme::for_display(&display);
        icon_theme.add_search_path("resources");
        if let Ok(dir) = std::env::current_dir() {
            icon_theme.add_search_path(dir.join("resources"));
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                icon_theme.add_search_path(dir.join("resources"));
                icon_theme.add_search_path(dir.join("../../resources"));
            }
        }
    }
}

pub fn run() -> Result<()> {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| setup_style_and_icons());

    let audio_engine_holder = Rc::new(RefCell::new(None));
    let engine_for_signal = audio_engine_holder.clone();
    let engine_slot = audio_engine_holder.clone();

    app.connect_activate(move |app| {
        let engine = match AudioEngine::new() {
            Ok(eng) => eng,
            Err(e) => {
                eprintln!("Warning: Failed to initialize PipeWire audio engine: {}", e);
                return;
            }
        };

        *engine_slot.borrow_mut() = Some(engine.clone());
        let window = MainWindow::new(app, engine);
        window.present();
    });

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
