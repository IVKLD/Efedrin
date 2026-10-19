use super::MainWindow;
use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

impl MainWindow {
    pub(super) fn open_folder_dialog(self: &Rc<Self>) {
        let dialog = gtk::FileChooserNative::builder()
            .title("Select Sounds Directory")
            .action(gtk::FileChooserAction::SelectFolder)
            .transient_for(&self.window)
            .modal(true)
            .accept_label("Open")
            .cancel_label("Cancel")
            .build();

        let this = self.clone();
        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        this.load_directory(&path);
                    }
                }
            }
        });

        dialog.show();
    }
}
