use super::scanner;
use super::MainWindow;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::Ordering;

impl MainWindow {
    pub fn load_directory(self: &Rc<Self>, path: &Path) {
        self.config.borrow_mut().sound_directory = Some(path.to_path_buf());
        let _ = self.config.borrow().save();
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("Sounds");
        self.controls.set_folder_label(name);
        scanner::setup_watcher(&self.watcher, path, self.scan_sender.clone());
        self.start_scan(path, true);
    }

    pub fn refresh_library(self: &Rc<Self>, is_initial: bool) {
        let dir = self.config.borrow().sound_directory.clone();
        if let Some(path) = dir {
            if path.exists() {
                self.start_scan(&path, is_initial);
            }
        }
    }

    pub(super) fn start_scan(self: &Rc<Self>, path: &Path, is_initial: bool) {
        let gen = self.load_generation.fetch_add(1, Ordering::SeqCst) + 1;
        if is_initial {
            self.count_label.set_label("Scanning...");
            self.status_label.set_label(&format!("Scanning: {}...", path.display()));
        }
        let existing = self.library.borrow().items().to_vec();
        scanner::spawn_scan(
            path.to_path_buf(),
            is_initial,
            gen,
            self.load_generation.clone(),
            existing,
            self.scan_sender.clone(),
        );
    }
}
