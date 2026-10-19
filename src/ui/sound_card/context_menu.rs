use crate::sound::SoundItem;
use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

pub fn attach_context_menu<F, E>(
    container: &gtk::Box,
    item: &SoundItem,
    on_play: Rc<F>,
    on_edit_tags: Rc<E>,
) where
    F: Fn(&SoundItem) + 'static,
    E: Fn(&SoundItem) + 'static,
{
    let popover = gtk::Popover::builder().has_arrow(false).autohide(true).build();
    let menu_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(4)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build();

    let play_btn = gtk::Button::builder()
        .label("Play / Stop")
        .icon_name("media-playback-start-symbolic")
        .css_classes(["flat"])
        .halign(gtk::Align::Fill)
        .build();

    let tags_btn = gtk::Button::builder()
        .label("Edit Tags...")
        .icon_name("tag-symbolic")
        .css_classes(["flat"])
        .halign(gtk::Align::Fill)
        .build();

    let folder_btn = gtk::Button::builder()
        .label("Open in File Manager")
        .icon_name("folder-open-symbolic")
        .css_classes(["flat"])
        .halign(gtk::Align::Fill)
        .build();

    menu_box.append(&play_btn);
    menu_box.append(&tags_btn);
    menu_box.append(&folder_btn);
    popover.set_child(Some(&menu_box));
    popover.set_parent(container);

    let pop1 = popover.clone();
    let item1 = item.clone();
    let cb1 = on_play;
    play_btn.connect_clicked(move |_| {
        pop1.popdown();
        cb1(&item1);
    });

    let pop2 = popover.clone();
    let item2 = item.clone();
    let cb2 = on_edit_tags;
    tags_btn.connect_clicked(move |_| {
        pop2.popdown();
        cb2(&item2);
    });

    let pop3 = popover.clone();
    let path = item.path.clone();
    folder_btn.connect_clicked(move |_| {
        pop3.popdown();
        if let Some(parent) = path.parent() {
            let _ = std::process::Command::new("xdg-open").arg(parent).spawn();
        }
    });

    let right_click = gtk::GestureClick::builder().button(3).build();
    let pop_open = popover;
    right_click.connect_pressed(move |_, _, x, y| {
        let rect = gtk4::gdk::Rectangle::new(x as i32, y as i32, 1, 1);
        pop_open.set_pointing_to(Some(&rect));
        pop_open.popup();
    });
    container.add_controller(right_click);
}
