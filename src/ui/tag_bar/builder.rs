use gtk4 as gtk;
use gtk4::prelude::*;

pub struct TagBarWidgets {
    pub scrolled: gtk::ScrolledWindow,
    pub buttons_box: gtk::Box,
}

pub fn create_tag_bar_widgets() -> TagBarWidgets {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .margin_start(16)
        .margin_end(16)
        .margin_top(4)
        .margin_bottom(6)
        .valign(gtk::Align::Center)
        .build();

    let tag_icon = gtk::Image::builder()
        .icon_name("tag-symbolic")
        .pixel_size(16)
        .css_classes(["dim-label"])
        .tooltip_text("Filter sounds by tag")
        .build();
    container.append(&tag_icon);

    let buttons_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .valign(gtk::Align::Center)
        .build();
    container.append(&buttons_box);

    let scrolled = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Never)
        .hexpand(true)
        .vexpand(false)
        .child(&container)
        .build();

    TagBarWidgets {
        scrolled,
        buttons_box,
    }
}
