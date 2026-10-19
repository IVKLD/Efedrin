#[cfg(test)]
mod tests {
    use super::super::LibraryWatcher;
    use notify::event::{AccessKind, CreateKind, ModifyKind};
    use notify::{Event, EventKind};
    use std::path::PathBuf;

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
        let unique_dir = std::env::temp_dir()
            .join(format!("efedrin_watch_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&unique_dir);
        std::fs::create_dir_all(&unique_dir).expect("create test dir");

        let (tx, rx) = std::sync::mpsc::channel();
        let watcher = LibraryWatcher::watch(&unique_dir, move || {
            let _ = tx.send(());
        });
        assert!(watcher.is_ok(), "Watcher should initialize on valid directory");

        let test_file = unique_dir.join("live_test.mp3");
        std::fs::write(&test_file, b"fake mp3 audio data").expect("write test file");

        let received = rx.recv_timeout(std::time::Duration::from_secs(2));
        assert!(received.is_ok(), "Watcher should trigger on created audio file");

        let _ = std::fs::remove_dir_all(&unique_dir);
    }
}
