use crate::sound::SoundItem;
use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

pub fn attach_play_controllers<F>(container: &gtk::Box, item: &SoundItem, on_play: Rc<F>)
where
    F: Fn(&SoundItem) + 'static,
{
    let left_click = gtk::GestureClick::builder().button(1).build();
    let item_for_play = item.clone();
    let on_play_cb = on_play.clone();
    let container_weak = container.downgrade();

    left_click.connect_pressed(move |_, _, x, y| {
        if let Some(cont) = container_weak.upgrade() {
            let mut current = cont.pick(x, y, gtk::PickFlags::DEFAULT);
            while let Some(w) = current {
                if w.is::<gtk::Button>() {
                    return;
                }
                current = w.parent();
            }
            on_play_cb(&item_for_play);
        }
    });
    container.add_controller(left_click);

    let key_ctrl = gtk::EventControllerKey::new();
    let item_for_key = item.clone();
    let on_play_key = on_play;
    key_ctrl.connect_key_pressed(move |_, keyval, _, _| {
        if keyval == gtk4::gdk::Key::Return || keyval == gtk4::gdk::Key::KP_Enter {
            on_play_key(&item_for_key);
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    container.add_controller(key_ctrl);
}
