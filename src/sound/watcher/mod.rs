mod filter;

#[cfg(test)]
mod tests;

use crate::error::{AppError, Result};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};

pub struct LibraryWatcher {
    _watcher: RecommendedWatcher,
    watched_path: PathBuf,
}

impl LibraryWatcher {
    pub fn watch<F>(path: &Path, on_change: F) -> Result<Self>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let path_buf = path.to_path_buf();
        let mut watcher = RecommendedWatcher::new(
            move |res: std::result::Result<Event, notify::Error>| match res {
                Ok(event) => {
                    if filter::is_relevant_event(&event) {
                        on_change();
                    }
                }
                Err(e) => eprintln!("Library watcher error: {}", e),
            },
            Config::default(),
        )
        .map_err(|e| AppError::Watcher(format!("Failed to create watcher: {}", e)))?;

        watcher
            .watch(path, RecursiveMode::Recursive)
            .map_err(|e| {
                AppError::Watcher(format!(
                    "Failed to watch directory {}: {}",
                    path.display(),
                    e
                ))
            })?;

        Ok(Self {
            _watcher: watcher,
            watched_path: path_buf,
        })
    }

    pub fn watched_path(&self) -> &Path {
        &self.watched_path
    }

    #[cfg(test)]
    pub fn is_relevant(event: &Event) -> bool {
        filter::is_relevant_event(event)
    }
}
