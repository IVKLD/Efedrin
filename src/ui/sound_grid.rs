use crate::sound::SoundItem;
use crate::ui::sound_card::SoundCardWidget;
use gtk4 as gtk;
use gtk4::prelude::*;
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
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();

        let flow_box = gtk::FlowBox::builder()
            .valign(gtk::Align::Start)
            .max_children_per_line(8)
            .min_children_per_line(2)
            .selection_mode(gtk::SelectionMode::None)
            .column_spacing(12)
            .row_spacing(12)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .build();

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .child(&flow_box)
            .build();

        let status_page = adw::StatusPage::builder()
            .icon_name("audio-x-generic-symbolic")
            .title("No Sounds Loaded")
            .description("Select a folder with audio files (.mp3, .wav, .ogg, .flac) to populate the soundboard.")
            .hexpand(true)
            .vexpand(true)
            .build();

        container.append(&status_page);
        container.append(&scrolled_window);
        scrolled_window.set_visible(false);

        Self {
            container,
            flow_box,
            status_page,
            scrolled_window,
            cards: RefCell::new(Vec::new()),
        }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.container
    }

    pub fn populate<F>(&self, sounds: &[SoundItem], on_play: Rc<F>, current_active_id: Option<&str>)
    where
        F: Fn(&SoundItem) + 'static,
    {
        while let Some(child) = self.flow_box.first_child() {
            self.flow_box.remove(&child);
        }
        self.cards.borrow_mut().clear();

        if sounds.is_empty() {
            self.status_page.set_visible(true);
            self.scrolled_window.set_visible(false);
        } else {
            self.status_page.set_visible(false);
            self.scrolled_window.set_visible(true);

            let mut new_cards = Vec::with_capacity(sounds.len());
            for item in sounds {
                let on_play_clone = on_play.clone();
                let card = SoundCardWidget::create(item, move |clicked_item| {
                    on_play_clone(clicked_item);
                });
                if current_active_id.map_or(false, |id| id == item.id) {
                    card.set_playing(true);
                }
                self.flow_box.insert(&card.button, -1);
                new_cards.push(card);
            }
            *self.cards.borrow_mut() = new_cards;
        }
    }

    pub fn set_active_sound(&self, active_id: Option<&str>) {
        for card in self.cards.borrow().iter() {
            let is_current = active_id.map_or(false, |id| id == card.item.id);
            card.set_playing(is_current);
        }
    }
}
