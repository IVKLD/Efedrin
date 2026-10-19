use gtk4 as gtk;
use gtk4::prelude::*;

pub struct ProgressWidgets {
    pub progress_box: gtk::Box,
    pub progress_scale: gtk::Scale,
    pub time_current_label: gtk::Label,
    pub time_total_label: gtk::Label,
}

#[inline]
pub fn format_timestamp(secs: f64) -> String {
    let s = secs.round() as u64;
    format!("{:02}:{:02}", s / 60, s % 60)
}

pub fn create_progress_box() -> ProgressWidgets {
    let progress_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(12)
        .build();

    let time_current_label = gtk::Label::builder()
        .label("00:00")
        .css_classes(["caption", "numeric"])
        .width_chars(5)
        .build();

    let progress_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 0.5);
    progress_scale.set_hexpand(true);
    progress_scale.set_draw_value(false);

    let time_total_label = gtk::Label::builder()
        .label("00:00")
        .css_classes(["caption", "numeric", "dim-label"])
        .width_chars(5)
        .build();

    progress_box.append(&time_current_label);
    progress_box.append(&progress_scale);
    progress_box.append(&time_total_label);

    ProgressWidgets {
        progress_box,
        progress_scale,
        time_current_label,
        time_total_label,
    }
}
