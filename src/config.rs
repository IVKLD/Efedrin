use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub sound_directory: Option<PathBuf>,
    pub volume_local: f64,
    pub volume_mic: f64,
    pub mic_passthrough: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            sound_directory: None,
            volume_local: 0.8,
            volume_mic: 1.0,
            mic_passthrough: false,
        }
    }
}

impl AppConfig {
    /// Returns the standard config directory path: ~/.config/efedrin
    pub fn config_dir() -> PathBuf {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.is_empty() {
                return PathBuf::from(xdg).join("efedrin");
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".config").join("efedrin");
        }
        PathBuf::from(".config").join("efedrin")
    }

    /// Returns the config file path: ~/.config/efedrin/config.json
    pub fn config_file() -> PathBuf {
        Self::config_dir().join("config.json")
    }

    /// Loads application configuration from disk or returns default if not found
    pub fn load() -> Self {
        let path = Self::config_file();
        if !path.exists() {
            return Self::default();
        }

        match File::open(&path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_default()
            }
            Err(_) => Self::default(),
        }
    }

    /// Persists configuration to disk atomically
    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let file = File::create(Self::config_file())?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
}
