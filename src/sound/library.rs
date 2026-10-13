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

        let id = path.to_string_lossy().into_owned();

        Some(Self {
            id,
            title,
            path: path.to_path_buf(),
            extension: ext.to_uppercase(),
            formatted_size,
            duration_secs: 0.0,
            formatted_duration: String::new(),
        })
    }

    pub fn update_duration(&mut self, secs: f64) {
        self.duration_secs = secs;
        self.formatted_duration = Self::format_duration(secs);
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

    pub fn scan_path(directory: &Path) -> Result<Vec<SoundItem>> {
        if !directory.is_dir() {
            return Err(AppError::NotFound(format!(
                "Directory does not exist: {}",
                directory.display()
            )));
        }

        let mut discovered: Vec<_> = WalkDir::new(directory)
            .follow_links(true)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| SoundItem::from_path(e.path()))
            .collect();

        discovered.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        Ok(discovered)
    }

    #[allow(dead_code)]
    pub fn scan_directory(&mut self, directory: &Path) -> Result<usize> {
        let items = Self::scan_path(directory)?;
        let count = items.len();
        self.items = items;
        Ok(count)
    }

    pub fn set_items(&mut self, items: Vec<SoundItem>) {
        self.items = items;
    }

    #[allow(dead_code)]
    pub fn items(&self) -> &[SoundItem] {
        &self.items
    }

    pub fn update_item_duration(&mut self, id: &str, duration_secs: f64) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.update_duration(duration_secs);
        }
    }

    pub fn filter(&self, query: &str) -> Vec<SoundItem> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return self.items.clone();
        }

        let query_lower = trimmed.to_lowercase();
        self.items
            .iter()
            .filter(|item| item.title.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_item_creation_and_duration_update() {
        let sample_path = Path::new("sample_sounds/coin.wav");
        if sample_path.exists() {
            let item = SoundItem::from_path(sample_path).expect("should parse sound item");
            assert_eq!(item.duration_secs, 0.0);
            assert_eq!(item.formatted_duration, "");
            assert_eq!(item.extension, "WAV");

            let mut library = SoundLibrary::new();
            library.set_items(vec![item.clone()]);
            library.update_item_duration(&item.id, 125.4);

            let updated = &library.items()[0];
            assert_eq!(updated.duration_secs, 125.4);
            assert_eq!(updated.formatted_duration, "02:05");
        }
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(SoundItem::format_duration(0.0), "00:00");
        assert_eq!(SoundItem::format_duration(59.4), "00:59");
        assert_eq!(SoundItem::format_duration(60.0), "01:00");
        assert_eq!(SoundItem::format_duration(3665.0), "61:05");
    }
}
