use super::types::AppConfig;
use std::path::Path;

impl AppConfig {
    pub fn tag_key_for_path(&self, path: &Path) -> String {
        if let Some(ref dir) = self.sound_directory {
            if let Ok(rel) = path.strip_prefix(dir) {
                return rel.to_string_lossy().to_string();
            }
        }
        path.to_string_lossy().to_string()
    }

    pub fn get_track_tags(&self, path: &Path) -> Vec<String> {
        let key = self.tag_key_for_path(path);
        if let Some(tags) = self.track_tags.get(&key) {
            return tags.clone();
        }
        let abs_key = path.to_string_lossy().to_string();
        self.track_tags.get(&abs_key).cloned().unwrap_or_default()
    }

    pub fn set_track_tags(&mut self, path: &Path, tags: Vec<String>) {
        let mut cleaned: Vec<String> = tags
            .into_iter()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();
        cleaned.sort();
        cleaned.dedup();

        let key = self.tag_key_for_path(path);
        let abs_key = path.to_string_lossy().to_string();

        if cleaned.is_empty() {
            self.track_tags.remove(&key);
            self.track_tags.remove(&abs_key);
        } else {
            self.track_tags.insert(key.clone(), cleaned);
            if key != abs_key {
                self.track_tags.remove(&abs_key);
            }
        }
    }
}
