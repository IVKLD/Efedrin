use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

pub fn create_input_section<F>(on_submit: F) -> (gtk::Box, gtk::Entry)
where
    F: Fn(String) + 'static,
{
    let add_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .build();

    let entry = gtk::Entry::builder()
        .placeholder_text("New tag name (e.g. meme, voice, sfx)...")
        .hexpand(true)
        .build();

    let add_btn = gtk::Button::builder()
        .label("Add")
        .icon_name("list-add-symbolic")
        .css_classes(["suggested-action"])
        .build();

    add_box.append(&entry);
    add_box.append(&add_btn);

    let on_submit = Rc::new(on_submit);
    let entry_clone = entry.clone();
    let submit_cb = on_submit.clone();
    let do_add = move || {
        let text = entry_clone.text().trim().to_string();
        let clean = text.strip_prefix('#').unwrap_or(&text).trim().to_string();
        if !clean.is_empty() {
            submit_cb(clean);
            entry_clone.set_text("");
        }
    };

    let do_add_rc = Rc::new(do_add);
    let cb1 = do_add_rc.clone();
    add_btn.connect_clicked(move |_| cb1());

    let cb2 = do_add_rc;
    entry.connect_activate(move |_| cb2());

    (add_box, entry)
}
