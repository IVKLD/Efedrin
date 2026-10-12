use gtk4::prelude::*;
use gtk4 as gtk;

pub struct ControlsBar {
    container: gtk::Box,
    folder_button: gtk::Button,
    folder_label: gtk::Label,
    search_entry: gtk::SearchEntry,
    vol_local_scale: gtk::Scale,
    vol_mic_scale: gtk::Scale,
    passthrough_switch: gtk::Switch,
}

impl ControlsBar {
    pub fn new(initial_vol_local: f64, initial_vol_mic: f64, initial_passthrough: bool) -> Self {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_start(16)
            .margin_end(16)
            .margin_top(12)
            .margin_bottom(8)
            .build();

        // Row 1: Folder Selector + Search Bar + Mic Passthrough Toggle
        let row1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(12)
            .build();

        // Folder button
        let folder_button = gtk::Button::builder()
            .tooltip_text("Open folder containing audio files")
            .css_classes(["flat"])
            .build();

        let folder_btn_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(6)
            .build();

        let folder_icon = gtk::Image::builder()
            .icon_name("folder-music-symbolic")
            .pixel_size(16)
            .build();

        let folder_label = gtk::Label::builder()
            .label("Choose Folder...")
            .ellipsize(gtk4::pango::EllipsizeMode::Middle)
            .max_width_chars(24)
            .build();

        folder_btn_box.append(&folder_icon);
        folder_btn_box.append(&folder_label);
        folder_button.set_child(Some(&folder_btn_box));

        // Search entry
        let search_entry = gtk::SearchEntry::builder()
            .placeholder_text("Search sounds...")
            .hexpand(true)
            .build();

        // Mic passthrough toggle box
        let passthrough_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .tooltip_text("Mix physical microphone input into virtual microphone")
            .build();

        let pt_icon = gtk::Image::builder()
            .icon_name("audio-input-microphone-symbolic")
            .pixel_size(16)
            .build();

        let pt_label = gtk::Label::builder()
            .label("Mix Real Mic")
            .css_classes(["caption", "heading"])
            .build();

        let passthrough_switch = gtk::Switch::builder()
            .active(initial_passthrough)
            .valign(gtk::Align::Center)
            .build();

        passthrough_box.append(&pt_icon);
        passthrough_box.append(&pt_label);
        passthrough_box.append(&passthrough_switch);

        row1.append(&folder_button);
        row1.append(&search_entry);
        row1.append(&passthrough_box);

        // Row 2: Dual Volume Sliders (Headphones & Virtual Mic)
        let row2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(24)
            .homogeneous(true)
            .build();

        // Local volume slider box
        let local_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .build();

        let local_icon = gtk::Image::builder()
            .icon_name("audio-headphones-symbolic")
            .pixel_size(16)
            .build();

        let local_title = gtk::Label::builder()
            .label("Headphones:")
            .css_classes(["caption"])
            .build();

        let vol_local_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.5, 0.05);
        vol_local_scale.set_value(initial_vol_local);
        vol_local_scale.set_hexpand(true);
        vol_local_scale.set_draw_value(false);

        let vol_local_val_label = gtk::Label::builder()
            .label(&format!("{:.0}%", initial_vol_local * 100.0))
            .width_chars(5)
            .css_classes(["caption", "numeric"])
            .build();

        local_box.append(&local_icon);
        local_box.append(&local_title);
        local_box.append(&vol_local_scale);
        local_box.append(&vol_local_val_label);

        // Virtual mic volume slider box
        let mic_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .build();

        let mic_icon = gtk::Image::builder()
            .icon_name("audio-volume-high-symbolic")
            .pixel_size(16)
            .build();

        let mic_title = gtk::Label::builder()
            .label("Virtual Mic:")
            .css_classes(["caption"])
            .build();

        let vol_mic_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 1.5, 0.05);
        vol_mic_scale.set_value(initial_vol_mic);
        vol_mic_scale.set_hexpand(true);
        vol_mic_scale.set_draw_value(false);

        let vol_mic_val_label = gtk::Label::builder()
            .label(&format!("{:.0}%", initial_vol_mic * 100.0))
            .width_chars(5)
            .css_classes(["caption", "numeric"])
            .build();

        mic_box.append(&mic_icon);
        mic_box.append(&mic_title);
        mic_box.append(&vol_mic_scale);
        mic_box.append(&vol_mic_val_label);

        row2.append(&local_box);
        row2.append(&mic_box);

        container.append(&row1);
        container.append(&row2);

        // Synchronize value labels with slider values
        let local_lbl = vol_local_val_label.clone();
        vol_local_scale.connect_value_changed(move |scale| {
            local_lbl.set_label(&format!("{:.0}%", scale.value() * 100.0));
        });

        let mic_lbl = vol_mic_val_label.clone();
        vol_mic_scale.connect_value_changed(move |scale| {
            mic_lbl.set_label(&format!("{:.0}%", scale.value() * 100.0));
        });

        Self {
            container,
            folder_button,
            folder_label,
            search_entry,
            vol_local_scale,
            vol_mic_scale,
            passthrough_switch,
        }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.container
    }

    pub fn folder_button(&self) -> &gtk::Button {
        &self.folder_button
    }

    pub fn set_folder_label(&self, text: &str) {
        self.folder_label.set_label(text);
    }

    pub fn search_entry(&self) -> &gtk::SearchEntry {
        &self.search_entry
    }

    pub fn vol_local_scale(&self) -> &gtk::Scale {
        &self.vol_local_scale
    }

    pub fn vol_mic_scale(&self) -> &gtk::Scale {
        &self.vol_mic_scale
    }

    pub fn passthrough_switch(&self) -> &gtk::Switch {
        &self.passthrough_switch
    }

    pub fn local_volume(&self) -> f64 {
        self.vol_local_scale.value()
    }

    pub fn mic_volume(&self) -> f64 {
        self.vol_mic_scale.value()
    }
}
