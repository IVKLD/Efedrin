use super::events::LibraryEvent;
use crate::sound::{LibraryWatcher, SoundItem, SoundLibrary};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

pub fn setup_watcher(
    watcher: &RefCell<Option<LibraryWatcher>>,
    path: &Path,
    sender: Sender<LibraryEvent>,
) {
    let need_new = match watcher.borrow().as_ref() {
        Some(w) => w.watched_path() != path,
        None => true,
    };
    if need_new {
        *watcher.borrow_mut() = None;
        if let Ok(w) = LibraryWatcher::watch(path, move || {
            let _ = sender.send(LibraryEvent::DirectoryChanged);
        }) {
            *watcher.borrow_mut() = Some(w);
        }
    }
}

pub fn spawn_scan(
    path: PathBuf,
    is_initial: bool,
    generation: u64,
    active_gen: Arc<AtomicU64>,
    existing: Vec<SoundItem>,
    sender: Sender<LibraryEvent>,
) {
    thread::spawn(move || {
        if generation != active_gen.load(Ordering::SeqCst) {
            return;
        }
        let event = match SoundLibrary::scan_path_with_cache(&path, Some(&existing)) {
            Ok(items) => LibraryEvent::ScanComplete {
                generation,
                items,
                folder_display: path.display().to_string(),
                is_initial,
            },
            Err(e) => LibraryEvent::ScanError {
                generation,
                error: e.to_string(),
            },
        };

        if generation == active_gen.load(Ordering::SeqCst) {
            let _ = sender.send(event);
        }
    });
}
