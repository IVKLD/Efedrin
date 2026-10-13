use crate::audio::MicrophoneSource;
use gtk4 as gtk;
use gtk4::prelude::*;

pub struct ControlsBar {
    container: gtk::Box,
    folder_button: gtk::Button,
    folder_label: gtk::Label,
    search_entry: gtk::SearchEntry,
    vol_master_scale: gtk::Scale,
    vol_local_scale: gtk::Scale,
    vol_mic_scale: gtk::Scale,
    passthrough_switch: gtk::Switch,
    mic_drop_down: gtk::DropDown,
    mic_sources: std::cell::RefCell<Vec<MicrophoneSource>>,
    progress_scale: gtk::Scale,
    time_current_label: gtk::Label,
    time_total_label: gtk::Label,
}

impl ControlsBar {
    fn create_volume_channel(
        icon_name: &str,
        title: &str,
        initial_vol: f64,
    ) -> (gtk::Box, gtk::Scale) {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .build();

        let icon = gtk::Image::builder()
            .icon_name(icon_name)
            .pixel_size(16)
            .build();

        let title_label = gtk::Label::builder()
            .label(title)
            .css_classes(["caption"])
            .build();

        let scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 10.0, 0.1);
        scale.set_value(initial_vol);
        scale.set_hexpand(true);
        scale.set_draw_value(false);

        let val_label = gtk::Label::builder()
            .label(&format!("{:.0}%", initial_vol * 100.0))
            .width_chars(6)
            .css_classes(["caption", "numeric"])
            .build();

        let val_lbl_clone = val_label.clone();
        scale.connect_value_changed(move |s| {
            val_lbl_clone.set_label(&format!("{:.0}%", s.value() * 100.0));
        });

        container.append(&icon);
        container.append(&title_label);
        container.append(&scale);
        container.append(&val_label);

        (container, scale)
    }

    pub fn new(
        initial_vol_master: f64,
        initial_vol_local: f64,
        initial_vol_mic: f64,
        initial_passthrough: bool,
        microphones: Vec<MicrophoneSource>,
        selected_mic_node: Option<&str>,
    ) -> Self {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_start(16)
            .margin_end(16)
            .margin_top(12)
            .margin_bottom(8)
            .build();

        let row1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(12)
            .build();

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

        let search_entry = gtk::SearchEntry::builder()
            .placeholder_text("Search sounds...")
            .hexpand(true)
            .build();

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
            .label("Mix Mic")
            .css_classes(["caption", "heading"])
            .build();

        let mic_names: Vec<&str> = microphones.iter().map(|m| m.description.as_str()).collect();
        let mic_string_list = gtk::StringList::new(&mic_names);
        let mic_drop_down = gtk::DropDown::builder()
            .model(&mic_string_list)
            .tooltip_text("Select physical microphone for passthrough")
            .build();

        if let Some(selected) = selected_mic_node {
            if let Some(idx) = microphones.iter().position(|m| m.node_name == selected) {
                mic_drop_down.set_selected(idx as u32);
            }
        }

        let passthrough_switch = gtk::Switch::builder()
            .active(initial_passthrough)
            .valign(gtk::Align::Center)
            .build();

        passthrough_box.append(&pt_icon);
        passthrough_box.append(&pt_label);
        passthrough_box.append(&mic_drop_down);
        passthrough_box.append(&passthrough_switch);

        row1.append(&folder_button);
        row1.append(&search_entry);
        row1.append(&passthrough_box);

        let row2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(18)
            .homogeneous(true)
            .build();

        let (master_box, vol_master_scale) =
            Self::create_volume_channel("media-playback-start-symbolic", "Sound Vol:", initial_vol_master);
        let (local_box, vol_local_scale) =
            Self::create_volume_channel("audio-headphones-symbolic", "Headphones:", initial_vol_local);
        let (mic_box, vol_mic_scale) =
            Self::create_volume_channel("audio-volume-high-symbolic", "Virtual Mic:", initial_vol_mic);

        row2.append(&master_box);
        row2.append(&local_box);
        row2.append(&mic_box);

        let progress_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(12)
            .build();

        let time_current_label = gtk::Label::builder()
            .label("00:00")
            .css_classes(["caption", "numeric"])
            .width_chars(5)
            .build();

        let progress_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 0.5);
        progress_scale.set_hexpand(true);
        progress_scale.set_draw_value(false);

        let time_total_label = gtk::Label::builder()
            .label("00:00")
            .css_classes(["caption", "numeric", "dim-label"])
            .width_chars(5)
            .build();

        progress_box.append(&time_current_label);
        progress_box.append(&progress_scale);
        progress_box.append(&time_total_label);

        container.append(&row1);
        container.append(&row2);
        container.append(&progress_box);

        Self {
            container,
            folder_button,
            folder_label,
            search_entry,
            vol_master_scale,
            vol_local_scale,
            vol_mic_scale,
            passthrough_switch,
            mic_drop_down,
            mic_sources: std::cell::RefCell::new(microphones),
            progress_scale,
            time_current_label,
            time_total_label,
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

    pub fn vol_master_scale(&self) -> &gtk::Scale {
        &self.vol_master_scale
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

    pub fn mic_drop_down(&self) -> &gtk::DropDown {
        &self.mic_drop_down
    }

    pub fn progress_scale(&self) -> &gtk::Scale {
        &self.progress_scale
    }

    pub fn master_volume(&self) -> f64 {
        self.vol_master_scale.value()
    }

    pub fn local_volume(&self) -> f64 {
        self.vol_local_scale.value()
    }

    pub fn mic_volume(&self) -> f64 {
        self.vol_mic_scale.value()
    }

    pub fn selected_microphone_node(&self) -> Option<String> {
        let idx = self.mic_drop_down.selected() as usize;
        self.mic_sources
            .borrow()
            .get(idx)
            .map(|m| m.node_name.clone())
    }

    #[inline]
    fn format_timestamp(secs: f64) -> String {
        let s = secs.round() as u64;
        format!("{:02}:{:02}", s / 60, s % 60)
    }

    pub fn set_playback_progress(&self, current_secs: f64, total_secs: f64) {
        let total = if total_secs > 0.1 { total_secs } else { 1.0 };
        let current = current_secs.clamp(0.0, total);

        self.progress_scale.set_range(0.0, total);
        self.progress_scale.set_value(current);
        self.time_current_label.set_label(&Self::format_timestamp(current));
        if total_secs > 0.1 {
            self.time_total_label.set_label(&Self::format_timestamp(total));
        } else {
            self.time_total_label.set_label("--:--");
        }
    }

    pub fn reset_progress(&self) {
        self.progress_scale.set_range(0.0, 100.0);
        self.progress_scale.set_value(0.0);
        self.time_current_label.set_label("00:00");
        self.time_total_label.set_label("00:00");
    }
}
