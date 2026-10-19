use super::events::LibraryEvent;
use super::MainWindow;
use gtk4::glib;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::time::Duration;

impl MainWindow {
    pub(super) fn handle_library_event(self: &Rc<Self>, event: LibraryEvent) {
        match event {
            LibraryEvent::DirectoryChanged => {
                let weak = Rc::downgrade(self);
                glib::timeout_add_local_once(Duration::from_millis(300), move || {
                    if let Some(this) = weak.upgrade() {
                        this.refresh_library(false);
                    }
                });
            }
            LibraryEvent::ScanComplete { generation, mut items, folder_display, is_initial } => {
                if generation != self.load_generation.load(Ordering::SeqCst) {
                    return;
                }

                {
                    let cfg = self.config.borrow();
                    for item in &mut items {
                        item.tags = cfg.get_track_tags(&item.path);
                    }
                }

                let items_changed = *self.library.borrow().items() != items;
                let count = items.len();
                self.library.borrow_mut().set_items(items.clone());

                let tag_counts = self.library.borrow().tag_counts();
                self.tag_bar.update_tags(count, &tag_counts);

                if is_initial || items_changed {
                    self.count_label.set_label(&format!("{} sounds loaded", count));
                    if is_initial {
                        self.status_label.set_label(&format!("Loaded folder: {}", folder_display));
                    } else {
                        self.status_label.set_label(&format!("Library updated: {} sounds", count));
                    }

                    if let Some(playing_id) = self.current_playing_id.borrow().as_ref() {
                        if !items.iter().any(|i| &i.id == playing_id) {
                            self.stop_all();
                            self.status_label.set_label("Currently playing sound was removed from library, playback stopped");
                        }
                    }
                    self.apply_filter();
                }
            }
            LibraryEvent::ScanError { generation, error } => {
                if generation != self.load_generation.load(Ordering::SeqCst) {
                    return;
                }
                self.status_label.set_label(&format!("Error scanning folder: {}", error));
                self.count_label.set_label("0 sounds loaded");
                self.render_sound_cards(&[]);
            }
        }
    }
}
