use gtk4::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;

pub struct Header {
    header_bar: adw::HeaderBar,
    stop_button: gtk::Button,
    #[allow(dead_code)]
    status_pill: gtk::Box,
}

impl Header {
    pub fn new() -> Self {
        let header_bar = adw::HeaderBar::new();

        let window_title = adw::WindowTitle::builder()
            .title("Efedrin Soundpad")
            .subtitle("PipeWire Virtual Microphone")
            .build();
        header_bar.set_title_widget(Some(&window_title));

        // Stop All button on start (left) side
        let stop_button = gtk::Button::builder()
            .icon_name("media-playback-stop-symbolic")
            .label("Stop All")
            .css_classes(["destructive-action"])
            .tooltip_text("Stop all currently playing audio (Space / Esc)")
            .build();
        header_bar.pack_start(&stop_button);

        // PipeWire indicator badge on end (right) side
        let status_pill = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(6)
            .css_classes(["badge", "pill"])
            .margin_end(6)
            .tooltip_text("PipeWire Virtual Microphone 'efedrin_mic' is active")
            .build();

        let dot = gtk::Image::builder()
            .icon_name("media-record-symbolic")
            .pixel_size(10)
            .css_classes(["accent"])
            .build();

        let pill_label = gtk::Label::builder()
            .label("efedrin_mic")
            .css_classes(["caption", "heading"])
            .build();

        status_pill.append(&dot);
        status_pill.append(&pill_label);

        header_bar.pack_end(&status_pill);

        Self {
            header_bar,
            stop_button,
            status_pill,
        }
    }

    pub fn widget(&self) -> &adw::HeaderBar {
        &self.header_bar
    }

    pub fn stop_button(&self) -> &gtk::Button {
        &self.stop_button
    }

    #[allow(dead_code)]
    pub fn status_pill(&self) -> &gtk::Box {
        &self.status_pill
    }
}
