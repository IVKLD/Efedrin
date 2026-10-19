mod builder;
mod populate;

use crate::sound::SoundItem;
use crate::ui::sound_card::SoundCardWidget;
use gtk4 as gtk;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SoundGrid {
    container: gtk::Box,
    flow_box: gtk::FlowBox,
    status_page: adw::StatusPage,
    scrolled_window: gtk::ScrolledWindow,
    cards: RefCell<Vec<SoundCardWidget>>,
}

impl SoundGrid {
    pub fn new() -> Self {
        let widgets = builder::create_grid_widgets();
        Self {
            container: widgets.container,
            flow_box: widgets.flow_box,
            status_page: widgets.status_page,
            scrolled_window: widgets.scrolled_window,
            cards: RefCell::new(Vec::new()),
        }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.container
    }

    pub fn populate<F, E, T>(
        &self,
        sounds: &[SoundItem],
        on_play: Rc<F>,
        on_edit_tags: Rc<E>,
        on_tag_clicked: Rc<T>,
        current_active_id: Option<&str>,
    ) where
        F: Fn(&SoundItem) + 'static,
        E: Fn(&SoundItem) + 'static,
        T: Fn(&str) + 'static,
    {
        let cards = populate::populate_grid(
            &self.flow_box,
            &self.status_page,
            &self.scrolled_window,
            sounds,
            on_play,
            on_edit_tags,
            on_tag_clicked,
            current_active_id,
        );
        *self.cards.borrow_mut() = cards;
    }

    pub fn set_active_sound(&self, active_id: Option<&str>) {
        for card in self.cards.borrow().iter() {
            card.set_playing(active_id == Some(&card.item.id));
        }
    }

    pub fn update_sound_duration(&self, sound_id: &str, secs: f64) {
        for card in self.cards.borrow_mut().iter_mut() {
            if card.item.id == sound_id {
                card.update_duration(secs);
                break;
            }
        }
    }
}
