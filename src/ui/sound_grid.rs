use crate::sound::SoundItem;
use crate::ui::sound_card::SoundCard;
use gtk4 as gtk;
use gtk4::prelude::*;
use libadwaita as adw;
use std::rc::Rc;

pub struct SoundGrid {
    container: gtk::Box,
    flow_box: gtk::FlowBox,
    status_page: adw::StatusPage,
    scrolled_window: gtk::ScrolledWindow,
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
        }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.container
    }

    pub fn populate<F>(&self, sounds: &[SoundItem], on_play: Rc<F>)
    where
        F: Fn(&SoundItem) + 'static,
    {
        while let Some(child) = self.flow_box.first_child() {
            self.flow_box.remove(&child);
        }

        if sounds.is_empty() {
            self.status_page.set_visible(true);
            self.scrolled_window.set_visible(false);
        } else {
            self.status_page.set_visible(false);
            self.scrolled_window.set_visible(true);

            for item in sounds {
                let on_play_clone = on_play.clone();
                let card = SoundCard::create(item, move |clicked_item| {
                    on_play_clone(clicked_item);
                });
                self.flow_box.insert(&card, -1);
            }
        }
    }
}
