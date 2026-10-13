use crate::sound::SoundItem;
use gtk4 as gtk;
use gtk4::prelude::*;

#[derive(Clone)]
pub struct SoundCardWidget {
    pub button: gtk::Button,
    pub icon: gtk::Image,
    pub badge: gtk::Label,
    pub size_label: gtk::Label,
    pub item: SoundItem,
}

impl SoundCardWidget {
    pub fn create<F>(item: &SoundItem, on_play: F) -> Self
    where
        F: Fn(&SoundItem) + 'static,
    {
        let button = gtk::Button::builder()
            .css_classes(["card", "sound-card-button"])
            .hexpand(true)
            .vexpand(false)
            .build();

        let root_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(6)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        let top_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();

        let icon = gtk::Image::builder()
            .icon_name("media-playback-start-symbolic")
            .pixel_size(18)
            .css_classes(["accent"])
            .build();

        let badge = gtk::Label::builder()
            .label(&item.extension)
            .css_classes(["badge", "caption", "dim-label"])
            .halign(gtk::Align::End)
            .hexpand(true)
            .justify(gtk::Justification::Center)
            .width_chars(7)
            .build();

        top_box.append(&icon);
        top_box.append(&badge);

        let title_label = gtk::Label::builder()
            .label(&item.title)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .max_width_chars(16)
            .halign(gtk::Align::Start)
            .css_classes(["heading"])
            .build();

        let subtitle = if item.duration_secs > 0.0 {
            format!("{} • {}", item.formatted_duration, item.formatted_size)
        } else {
            item.formatted_size.clone()
        };

        let size_label = gtk::Label::builder()
            .label(&subtitle)
            .halign(gtk::Align::Start)
            .css_classes(["caption", "dim-label"])
            .build();

        root_box.append(&top_box);
        root_box.append(&title_label);
        root_box.append(&size_label);

        button.set_child(Some(&root_box));

        let item_clone = item.clone();
        button.connect_clicked(move |_| {
            on_play(&item_clone);
        });

        let gesture = gtk::GestureClick::builder().button(3).build();

        let path_clone = item.path.clone();
        gesture.connect_pressed(move |_, _, _, _| {
            if let Some(parent) = path_clone.parent() {
                let _ = std::process::Command::new("xdg-open").arg(parent).spawn();
            }
        });
        button.add_controller(gesture);

        Self {
            button,
            icon,
            badge,
            size_label,
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
            self.button.add_css_class("is-playing");
            self.icon.set_icon_name(Some("audio-volume-high-symbolic"));
            self.badge.set_label("PLAYING");
            self.badge.add_css_class("badge-active");
        } else {
            self.button.remove_css_class("is-playing");
            self.icon
                .set_icon_name(Some("media-playback-start-symbolic"));
            self.badge.set_label(&self.item.extension);
            self.badge.remove_css_class("badge-active");
        }
    }
}
