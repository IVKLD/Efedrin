use super::item::SoundItem;
use crate::error::{AppError, Result};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_path_with_cache(
    directory: &Path,
    existing_items: Option<&[SoundItem]>,
) -> Result<Vec<SoundItem>> {
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

    if let Some(existing) = existing_items {
        let mut cache = HashMap::new();
        for item in existing {
            if item.duration_secs > 0.0 || !item.tags.is_empty() {
                cache.insert(
                    item.id.as_str(),
                    (
                        item.duration_secs,
                        item.formatted_duration.as_str(),
                        item.formatted_size.as_str(),
                        item.tags.clone(),
                    ),
                );
            }
        }

        for item in &mut discovered {
            if let Some((dur, fmt, size, tags)) = cache.get(item.id.as_str()) {
                if item.formatted_size == *size {
                    item.duration_secs = *dur;
                    item.formatted_duration = fmt.to_string();
                }
                item.tags = tags.clone();
            }
        }
    }

    discovered.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    Ok(discovered)
}
