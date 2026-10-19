use super::types::AppConfig;
use crate::error::Result;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

impl AppConfig {
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

    pub fn config_file() -> PathBuf {
        Self::config_dir().join("config.json")
    }

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
