use crate::error::{AppError, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq)]
pub struct SoundItem {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
    pub extension: String,
    pub formatted_size: String,
    pub duration_secs: f64,
    pub formatted_duration: String,
}

impl SoundItem {
    pub fn probe_duration(path: &Path) -> f64 {
        let output = match std::process::Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
            ])
            .arg(path)
            .output()
        {
            Ok(out) => out,
            Err(_) => return 0.0,
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.trim().parse::<f64>().unwrap_or(0.0)
    }

    pub fn format_duration(secs: f64) -> String {
        let total_secs = secs.round() as u64;
        let m = total_secs / 60;
        let s = total_secs % 60;
        format!("{:02}:{:02}", m, s)
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        let valid_extensions = ["mp3", "wav", "ogg", "flac", "opus", "m4a", "aac", "webm"];

        if !valid_extensions.contains(&ext.as_str()) {
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

        let duration_secs = Self::probe_duration(path);
        let formatted_duration = Self::format_duration(duration_secs);

        let id = path.to_string_lossy().to_string();

        Some(Self {
            id,
            title,
            path: path.to_path_buf(),
            extension: ext.to_uppercase(),
            formatted_size,
            duration_secs,
            formatted_duration,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct SoundLibrary {
    items: Vec<SoundItem>,
}

impl SoundLibrary {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn scan_directory(&mut self, directory: &Path) -> Result<usize> {
        if !directory.exists() || !directory.is_dir() {
            return Err(AppError::NotFound(format!(
                "Directory does not exist: {}",
                directory.display()
            )));
        }

        let mut discovered = Vec::new();

        for entry in WalkDir::new(directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(item) = SoundItem::from_path(path) {
                    discovered.push(item);
                }
            }
        }

        discovered.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        let count = discovered.len();
        self.items = discovered;

        Ok(count)
    }

    pub fn items(&self) -> &[SoundItem] {
        &self.items
    }

    pub fn filter(&self, query: &str) -> Vec<SoundItem> {
        if query.trim().is_empty() {
            return self.items.clone();
        }

        let query_lower = query.to_lowercase();
        self.items
            .iter()
            .filter(|item| item.title.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }
}
