use crate::sound::SoundItem;
use gtk4 as gtk;
use gtk4::prelude::*;

pub struct SoundCard;

impl SoundCard {
    pub fn create<F>(item: &SoundItem, on_play: F) -> gtk::Button
    where
        F: Fn(&SoundItem) + 'static,
    {
        let button = gtk::Button::builder()
            .css_classes(["card", "sound-card-button"])
            .hexpand(true)
            .vexpand(false)
            .build();

        let root_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(6)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        let top_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();

        let icon = gtk::Image::builder()
            .icon_name("media-playback-start-symbolic")
            .pixel_size(18)
            .css_classes(["accent"])
            .build();

        let badge = gtk::Label::builder()
            .label(&item.extension)
            .css_classes(["badge", "caption", "dim-label"])
            .halign(gtk::Align::End)
            .hexpand(true)
            .build();

        top_box.append(&icon);
        top_box.append(&badge);

        let title_label = gtk::Label::builder()
            .label(&item.title)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .max_width_chars(16)
            .halign(gtk::Align::Start)
            .css_classes(["heading"])
            .build();

        let size_label = gtk::Label::builder()
            .label(&item.formatted_size)
            .halign(gtk::Align::Start)
            .css_classes(["caption", "dim-label"])
            .build();

        root_box.append(&top_box);
        root_box.append(&title_label);
        root_box.append(&size_label);

        button.set_child(Some(&root_box));

        let item_clone = item.clone();
        button.connect_clicked(move |_| {
            on_play(&item_clone);
        });

        let gesture = gtk::GestureClick::builder().button(3).build();

        let path_clone = item.path.clone();
        gesture.connect_pressed(move |_, _, _, _| {
            if let Some(parent) = path_clone.parent() {
                let _ = std::process::Command::new("xdg-open").arg(parent).spawn();
            }
        });
        button.add_controller(gesture);

        button
    }
}
