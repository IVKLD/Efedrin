use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct SoundItem {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
    pub extension: String,
    pub formatted_size: String,
    pub duration_secs: f64,
    pub formatted_duration: String,
    pub tags: Vec<String>,
}

impl SoundItem {
    pub const SUPPORTED_EXTENSIONS: &[&str] = &[
        "mp3", "wav", "ogg", "flac", "opus", "m4a", "aac", "webm",
    ];

    pub fn is_supported_extension(ext: &str) -> bool {
        let ext = ext.to_lowercase();
        Self::SUPPORTED_EXTENSIONS.contains(&ext.as_str())
    }

    pub fn probe_duration(path: &Path) -> f64 {
        let mut cmd = std::process::Command::new("ffprobe");
        cmd.env_remove("LD_LIBRARY_PATH");
        cmd.args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(path)
        .output()
        .ok()
        .and_then(|out| String::from_utf8_lossy(&out.stdout).trim().parse::<f64>().ok())
        .unwrap_or(0.0)
    }

    pub fn format_duration(secs: f64) -> String {
        let total_secs = secs.round() as u64;
        let m = total_secs / 60;
        let s = total_secs % 60;
        format!("{:02}:{:02}", m, s)
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        if !Self::is_supported_extension(&ext) {
            return None;
        }

        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let size_bytes = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        let formatted_size = if size_bytes < 1024 * 1024 {
            format!("{:.1} KB", size_bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", size_bytes as f64 / (1024.0 * 1024.0))
        };

        let id = path.to_string_lossy().into_owned();

        Some(Self {
            id,
            title,
            path: path.to_path_buf(),
            extension: ext.to_uppercase(),
            formatted_size,
            duration_secs: 0.0,
            formatted_duration: String::new(),
            tags: Vec::new(),
        })
    }

    pub fn update_duration(&mut self, secs: f64) {
        self.duration_secs = secs;
        self.formatted_duration = Self::format_duration(secs);
    }
}
