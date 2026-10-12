mod app;
mod audio;
mod config;
mod error;
mod sound;
mod ui;

fn setup_environment() {
    let mut schema_dirs = Vec::new();
    let mut schema_dir_direct = None;

    if let Ok(entries) = std::fs::read_dir("/nix/store") {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name();
            let s = name.to_string_lossy();
            if (s.contains("-gtk4-") || s.contains("-gsettings-desktop-schemas-"))
                && !s.ends_with(".drv")
            {
                let share_schemas = entry.path().join("share/gsettings-schemas");
                if share_schemas.exists() {
                    if let Ok(sub) = std::fs::read_dir(&share_schemas) {
                        for sub_entry in sub.filter_map(|e| e.ok()) {
                            let p = sub_entry.path().join("glib-2.0/schemas/gschemas.compiled");
                            if p.exists() {
                                if schema_dir_direct.is_none() {
                                    schema_dir_direct =
                                        Some(sub_entry.path().join("glib-2.0/schemas"));
                                }
                                schema_dirs.push(sub_entry.path().to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    if !schema_dirs.is_empty() {
        let current_xdg = std::env::var("XDG_DATA_DIRS").unwrap_or_default();
        let new_xdg = if current_xdg.is_empty() {
            schema_dirs.join(":")
        } else {
            format!("{}:{}", schema_dirs.join(":"), current_xdg)
        };
        std::env::set_var("XDG_DATA_DIRS", new_xdg);
    }

    if let Some(dir) = schema_dir_direct {
        if std::env::var("GSETTINGS_SCHEMA_DIR").is_err() {
            std::env::set_var("GSETTINGS_SCHEMA_DIR", dir);
        }
    }
}

fn main() {
    setup_environment();
    if let Err(e) = app::run() {
        eprintln!("Fatal error in Efedrin Soundpad: {}", e);
        std::process::exit(1);
    }
}
