use crate::sound::SoundItem;
use gtk4 as gtk;
use gtk4::prelude::*;

pub struct PlayAreaWidgets {
    pub play_area: gtk::Box,
    pub icon: gtk::Image,
    pub badge: gtk::Label,
    pub size_label: gtk::Label,
}

pub fn create_play_area(item: &SoundItem) -> PlayAreaWidgets {
    let play_area = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(8)
        .margin_start(10)
        .margin_end(10)
        .build();
    play_area.set_cursor_from_name(Some("pointer"));

    let top_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
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
        .justify(gtk::Justification::Center)
        .width_chars(6)
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

    let subtitle = if item.duration_secs > 0.0 {
        format!("{} • {}", item.formatted_duration, item.formatted_size)
    } else {
        item.formatted_size.clone()
    };

    let size_label = gtk::Label::builder()
        .label(&subtitle)
        .halign(gtk::Align::Start)
        .css_classes(["caption", "dim-label"])
        .build();

    play_area.append(&top_box);
    play_area.append(&title_label);
    play_area.append(&size_label);

    PlayAreaWidgets {
        play_area,
        icon,
        badge,
        size_label,
    }
}
