mod app;
mod audio;
mod config;
mod error;
mod sound;
mod ui;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Fatal error in Efedrin Soundpad: {}", e);
        std::process::exit(1);
    }
}
