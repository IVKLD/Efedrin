use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

pub fn render_assigned_chips<F>(tags_flow: &gtk::FlowBox, tags: &[String], on_remove: Rc<F>)
where
    F: Fn(&str) + 'static,
{
    while let Some(child) = tags_flow.first_child() {
        tags_flow.remove(&child);
    }

    if tags.is_empty() {
        let empty_label = gtk::Label::builder()
            .label("No tags assigned yet. Add one below.")
            .css_classes(["caption", "dim-label"])
            .halign(gtk::Align::Start)
            .build();
        tags_flow.insert(&empty_label, -1);
        return;
    }

    for tag in tags {
        let color_class = crate::ui::tag_color_class(tag);
        let pill_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(6)
            .valign(gtk::Align::Center)
            .css_classes(["tag-chip", color_class])
            .build();

        let label = gtk::Label::builder().label(tag).valign(gtk::Align::Center).build();

        let remove_icon = gtk::Image::builder()
            .icon_name("window-close-symbolic")
            .pixel_size(12)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();

        let remove_btn = gtk::Button::builder()
            .child(&remove_icon)
            .css_classes(["flat", "circular", "tag-remove-btn"])
            .tooltip_text("Remove tag")
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();

        let tag_name = tag.clone();
        let cb = on_remove.clone();
        remove_btn.connect_clicked(move |_| cb(&tag_name));

        pill_box.append(&label);
        pill_box.append(&remove_btn);
        tags_flow.insert(&pill_box, -1);
    }
}
