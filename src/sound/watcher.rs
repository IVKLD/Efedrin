use crate::error::{AppError, Result};
use crate::sound::SoundItem;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
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
                    if Self::is_relevant(&event) {
                        on_change();
                    }
                }
                Err(e) => {
                    eprintln!("Library watcher error: {}", e);
                }
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

    pub fn is_relevant(event: &Event) -> bool {
        match event.kind {
            EventKind::Access(_) | EventKind::Other => false,
            _ => {
                if event.paths.is_empty() {
                    return true;
                }
                event.paths.iter().any(|path| {
                    if path.is_dir() {
                        return true;
                    }
                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        SoundItem::is_supported_extension(ext)
                    } else {
                        // Path with no extension could be a directory (new or deleted)
                        true
                    }
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{AccessKind, CreateKind, ModifyKind};

    #[test]
    fn test_is_relevant_ignores_access() {
        let event = Event {
            kind: EventKind::Access(AccessKind::Read),
            paths: vec![PathBuf::from("/tmp/sound.mp3")],
            attrs: Default::default(),
        };
        assert!(!LibraryWatcher::is_relevant(&event));
    }

    #[test]
    fn test_is_relevant_accepts_supported_audio_create() {
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/tmp/sound.ogg")],
            attrs: Default::default(),
        };
        assert!(LibraryWatcher::is_relevant(&event));
    }

    #[test]
    fn test_is_relevant_ignores_unsupported_extensions() {
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/tmp/document.pdf")],
            attrs: Default::default(),
        };
        assert!(!LibraryWatcher::is_relevant(&event));
    }

    #[test]
    fn test_is_relevant_accepts_directory_or_extensionless() {
        let event = Event {
            kind: EventKind::Modify(ModifyKind::Any),
            paths: vec![PathBuf::from("/tmp/subfolder")],
            attrs: Default::default(),
        };
        assert!(LibraryWatcher::is_relevant(&event));
    }

    #[test]
    fn test_live_directory_watcher() {
        let unique_dir = std::env::temp_dir().join(format!("efedrin_watch_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&unique_dir);
        std::fs::create_dir_all(&unique_dir).expect("create test dir");

        let (tx, rx) = std::sync::mpsc::channel();
        let watcher = LibraryWatcher::watch(&unique_dir, move || {
            let _ = tx.send(());
        });
        assert!(watcher.is_ok(), "Watcher should initialize on valid directory");

        // Create an audio file in the watched directory
        let test_file = unique_dir.join("live_test.mp3");
        std::fs::write(&test_file, b"fake mp3 audio data").expect("write test file");

        // Wait for event with timeout
        let received = rx.recv_timeout(std::time::Duration::from_secs(2));
        assert!(received.is_ok(), "Watcher should trigger on created audio file");

        let _ = std::fs::remove_dir_all(&unique_dir);
    }
}
