use gtk4 as gtk;
use gtk4::prelude::*;

pub fn create_volume_channel(
    icon_name: &str,
    title: &str,
    initial_vol: f64,
) -> (gtk::Box, gtk::Scale) {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .build();

    let icon = gtk::Image::builder()
        .icon_name(icon_name)
        .pixel_size(16)
        .build();

    let title_label = gtk::Label::builder()
        .label(title)
        .css_classes(["caption"])
        .build();

    let initial_vol = initial_vol.clamp(0.0, 4.0);
    let scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 4.0, 0.05);
    scale.set_value(initial_vol);
    scale.set_hexpand(true);
    scale.set_draw_value(false);

    let val_label = gtk::Label::builder()
        .label(&format!("{:.0}%", initial_vol * 100.0))
        .width_chars(6)
        .css_classes(["caption", "numeric"])
        .build();

    let val_lbl_clone = val_label.clone();
    scale.connect_value_changed(move |s| {
        val_lbl_clone.set_label(&format!("{:.0}%", s.value() * 100.0));
    });

    container.append(&icon);
    container.append(&title_label);
    container.append(&scale);
    container.append(&val_label);

    (container, scale)
}
