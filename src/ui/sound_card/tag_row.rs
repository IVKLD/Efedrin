use crate::sound::SoundItem;
use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

pub fn create_tag_row<E, T>(
    item: &SoundItem,
    on_edit_tags: Rc<E>,
    on_tag_clicked: Rc<T>,
) -> gtk::Box
where
    E: Fn(&SoundItem) + 'static,
    T: Fn(&str) + 'static,
{
    let bottom_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(4)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(8)
        .valign(gtk::Align::Center)
        .build();

    let tags_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(4)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();

    if item.tags.is_empty() {
        let add_btn = gtk::Button::builder()
            .label("+ tag")
            .icon_name("tag-symbolic")
            .css_classes(["flat", "tag-btn-add"])
            .tooltip_text("Add tags to this sound")
            .valign(gtk::Align::Center)
            .build();
        let it = item.clone();
        let cb = on_edit_tags.clone();
        add_btn.connect_clicked(move |_| cb(&it));
        bottom_row.append(&add_btn);
    } else {
        let max_tags = 2;
        for tag in item.tags.iter().take(max_tags) {
            let btn = gtk::Button::builder()
                .label(&format!("#{}", tag))
                .css_classes(["flat", "tag-pill", crate::ui::tag_color_class(tag)])
                .tooltip_text(&format!("Filter by tag '{}'", tag))
                .valign(gtk::Align::Center)
                .build();
            let t = tag.clone();
            let cb = on_tag_clicked.clone();
            btn.connect_clicked(move |_| cb(&t));
            tags_box.append(&btn);
        }

        if item.tags.len() > max_tags {
            let more_btn = gtk::Button::builder()
                .label(&format!("+{}", item.tags.len() - max_tags))
                .css_classes(["flat", "tag-pill"])
                .tooltip_text(&item.tags.join(", "))
                .valign(gtk::Align::Center)
                .build();
            let it = item.clone();
            let cb = on_edit_tags.clone();
            more_btn.connect_clicked(move |_| cb(&it));
            tags_box.append(&more_btn);
        } else {
            let edit_btn = gtk::Button::builder()
                .icon_name("tag-symbolic")
                .css_classes(["flat", "circular"])
                .tooltip_text("Edit tags")
                .valign(gtk::Align::Center)
                .build();
            let it = item.clone();
            let cb = on_edit_tags.clone();
            edit_btn.connect_clicked(move |_| cb(&it));
            tags_box.append(&edit_btn);
        }
        bottom_row.append(&tags_box);
    }

    bottom_row
}
