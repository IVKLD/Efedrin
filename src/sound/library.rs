use crate::error::{AppError, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundItem {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
    pub extension: String,
    pub formatted_size: String,
}

impl SoundItem {
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

        let id = path.to_string_lossy().to_string();

        Some(Self {
            id,
            title,
            path: path.to_path_buf(),
            extension: ext.to_uppercase(),
            formatted_size,
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

    /// Recursively scans a directory for audio files
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

    /// Filters sound items matching query string
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
