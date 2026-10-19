use crate::sound::SoundItem;
use crate::ui::sound_card::SoundCardWidget;
use gtk4 as gtk;
use gtk4::prelude::*;
use libadwaita as adw;
use std::rc::Rc;

pub fn populate_grid<F, E, T>(
    flow_box: &gtk::FlowBox,
    status_page: &adw::StatusPage,
    scrolled_window: &gtk::ScrolledWindow,
    sounds: &[SoundItem],
    on_play: Rc<F>,
    on_edit_tags: Rc<E>,
    on_tag_clicked: Rc<T>,
    current_active_id: Option<&str>,
) -> Vec<SoundCardWidget>
where
    F: Fn(&SoundItem) + 'static,
    E: Fn(&SoundItem) + 'static,
    T: Fn(&str) + 'static,
{
    while let Some(child) = flow_box.first_child() {
        flow_box.remove(&child);
    }

    let is_empty = sounds.is_empty();
    status_page.set_visible(is_empty);
    scrolled_window.set_visible(!is_empty);

    if is_empty {
        return Vec::new();
    }

    let mut cards = Vec::with_capacity(sounds.len());
    for item in sounds {
        let on_play_cb = on_play.clone();
        let on_edit_cb = on_edit_tags.clone();
        let on_tag_cb = on_tag_clicked.clone();
        let card = SoundCardWidget::create(
            item,
            move |clicked| on_play_cb(clicked),
            move |clicked| on_edit_cb(clicked),
            move |tag| on_tag_cb(tag),
        );
        if current_active_id == Some(&item.id) {
            card.set_playing(true);
        }
        flow_box.insert(&card.container, -1);
        cards.push(card);
    }
    cards
}
