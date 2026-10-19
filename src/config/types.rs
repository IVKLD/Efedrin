use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub sound_directory: Option<PathBuf>,
    pub volume_master: f64,
    pub volume_local: f64,
    pub volume_mic: f64,
    pub mic_passthrough: bool,
    pub selected_microphone: Option<String>,
    #[serde(default)]
    pub track_tags: HashMap<String, Vec<String>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            sound_directory: None,
            volume_master: 1.0,
            volume_local: 0.8,
            volume_mic: 1.0,
            mic_passthrough: false,
            selected_microphone: None,
            track_tags: HashMap::new(),
        }
    }
}
