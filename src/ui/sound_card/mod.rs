mod context_menu;
mod controllers;
mod play_area;
mod tag_row;

use crate::sound::SoundItem;
use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct SoundCardWidget {
    pub container: gtk::Box,
    pub icon: gtk::Image,
    pub badge: gtk::Label,
    pub size_label: gtk::Label,
    pub item: SoundItem,
}

impl SoundCardWidget {
    pub fn create<F, E, T>(item: &SoundItem, on_play: F, on_edit_tags: E, on_tag_clicked: T) -> Self
    where
        F: Fn(&SoundItem) + 'static,
        E: Fn(&SoundItem) + 'static,
        T: Fn(&str) + 'static,
    {
        let on_play = Rc::new(on_play);
        let on_edit_tags = Rc::new(on_edit_tags);
        let on_tag_clicked = Rc::new(on_tag_clicked);

        let container = gtk::Box::builder()
            .css_classes(["card", "sound-card-button"])
            .orientation(gtk::Orientation::Vertical)
            .spacing(4)
            .hexpand(true)
            .vexpand(false)
            .can_focus(true)
            .focusable(true)
            .build();
        container.set_cursor_from_name(Some("pointer"));

        let play_widgets = play_area::create_play_area(item);
        let bottom_row = tag_row::create_tag_row(item, on_edit_tags.clone(), on_tag_clicked);

        container.append(&play_widgets.play_area);
        container.append(&bottom_row);

        controllers::attach_play_controllers(&container, item, on_play.clone());
        context_menu::attach_context_menu(&container, item, on_play, on_edit_tags);

        Self {
            container,
            icon: play_widgets.icon,
            badge: play_widgets.badge,
            size_label: play_widgets.size_label,
            item: item.clone(),
        }
    }

    pub fn update_duration(&mut self, secs: f64) {
        self.item.update_duration(secs);
        let subtitle = if self.item.duration_secs > 0.0 {
            format!("{} • {}", self.item.formatted_duration, self.item.formatted_size)
        } else {
            self.item.formatted_size.clone()
        };
        self.size_label.set_label(&subtitle);
    }

    pub fn set_playing(&self, playing: bool) {
        if playing {
            self.container.add_css_class("is-playing");
            self.icon.set_icon_name(Some("audio-volume-high-symbolic"));
            self.badge.set_label("PLAYING");
            self.badge.add_css_class("badge-active");
        } else {
            self.container.remove_css_class("is-playing");
            self.icon.set_icon_name(Some("media-playback-start-symbolic"));
            self.badge.set_label(&self.item.extension);
            self.badge.remove_css_class("badge-active");
        }
    }
}
