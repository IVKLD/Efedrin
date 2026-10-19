use gtk4 as gtk;
use gtk4::prelude::*;

pub fn create_folder_button() -> (gtk::Button, gtk::Label) {
    let folder_button = gtk::Button::builder()
        .tooltip_text("Open folder containing audio files")
        .css_classes(["flat"])
        .build();

    let folder_btn_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .build();

    let folder_icon = gtk::Image::builder()
        .icon_name("folder-music-symbolic")
        .pixel_size(16)
        .build();

    let folder_label = gtk::Label::builder()
        .label("Choose Folder...")
        .ellipsize(gtk4::pango::EllipsizeMode::Middle)
        .max_width_chars(24)
        .build();

    folder_btn_box.append(&folder_icon);
    folder_btn_box.append(&folder_label);
    folder_button.set_child(Some(&folder_btn_box));

    (folder_button, folder_label)
}
