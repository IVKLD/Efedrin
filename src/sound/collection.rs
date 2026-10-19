use super::item::SoundItem;
use super::scanner;
use crate::error::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct SoundLibrary {
    items: Vec<SoundItem>,
}

impl SoundLibrary {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }


    pub fn scan_path_with_cache(
        directory: &Path,
        existing: Option<&[SoundItem]>,
    ) -> Result<Vec<SoundItem>> {
        scanner::scan_path_with_cache(directory, existing)
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

    pub fn set_item_tags(&mut self, id: &str, tags: Vec<String>) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.tags = tags;
        }
    }

    pub fn all_tags(&self) -> Vec<String> {
        let mut set = BTreeSet::new();
        for item in &self.items {
            for tag in &item.tags {
                set.insert(tag.clone());
            }
        }
        set.into_iter().collect()
    }

    pub fn tag_counts(&self) -> Vec<(String, usize)> {
        let mut counts = BTreeMap::new();
        for item in &self.items {
            for tag in &item.tags {
                *counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        counts.into_iter().collect()
    }

    #[allow(dead_code)]
    pub fn filter(&self, query: &str) -> Vec<SoundItem> {
        self.filter_with_tag(query, None)
    }

    pub fn filter_with_tag(&self, query: &str, selected_tag: Option<&str>) -> Vec<SoundItem> {
        let trimmed = query.trim().to_lowercase();
        let clean = trimmed.strip_prefix('#').unwrap_or(&trimmed);

        self.items
            .iter()
            .filter(|item| {
                if let Some(tag) = selected_tag {
                    if !tag.is_empty() && !item.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)) {
                        return false;
                    }
                }
                if clean.is_empty() {
                    return true;
                }
                item.title.to_lowercase().contains(clean)
                    || item.tags.iter().any(|t| t.to_lowercase().contains(clean))
            })
            .cloned()
            .collect()
    }
}
