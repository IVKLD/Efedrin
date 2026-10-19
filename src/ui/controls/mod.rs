mod builder;
mod folder;
mod passthrough;
mod progress;
mod volume;

use crate::audio::MicrophoneSource;
use gtk4 as gtk;
use gtk4::prelude::*;
use std::cell::RefCell;

pub struct ControlsBar {
    b: builder::BuiltControls,
    mic_sources: RefCell<Vec<MicrophoneSource>>,
}

impl ControlsBar {
    pub fn new(
        v_master: f64, v_local: f64, v_mic: f64, pt: bool,
        mics: Vec<MicrophoneSource>, selected_mic: Option<&str>,
    ) -> Self {
        let b = builder::build_controls(v_master, v_local, v_mic, pt, &mics, selected_mic);
        Self { b, mic_sources: RefCell::new(mics) }
    }

    pub fn widget(&self) -> &gtk::Box { &self.b.container }
    pub fn folder_button(&self) -> &gtk::Button { &self.b.folder_button }
    pub fn set_folder_label(&self, text: &str) { self.b.folder_label.set_label(text); }
    pub fn search_entry(&self) -> &gtk::SearchEntry { &self.b.search_entry }
    pub fn vol_master_scale(&self) -> &gtk::Scale { &self.b.vol_master_scale }
    pub fn vol_local_scale(&self) -> &gtk::Scale { &self.b.vol_local_scale }
    pub fn vol_mic_scale(&self) -> &gtk::Scale { &self.b.vol_mic_scale }
    pub fn passthrough_switch(&self) -> &gtk::Switch { &self.b.passthrough_switch }
    pub fn mic_drop_down(&self) -> &gtk::DropDown { &self.b.mic_drop_down }
    pub fn progress_scale(&self) -> &gtk::Scale { &self.b.progress_scale }
    pub fn master_volume(&self) -> f64 { self.b.vol_master_scale.value() }
    pub fn local_volume(&self) -> f64 { self.b.vol_local_scale.value() }
    pub fn mic_volume(&self) -> f64 { self.b.vol_mic_scale.value() }

    pub fn selected_microphone_node(&self) -> Option<String> {
        let idx = self.b.mic_drop_down.selected() as usize;
        self.mic_sources.borrow().get(idx).map(|m| m.node_name.clone())
    }

    pub fn set_playback_progress(&self, current_secs: f64, total_secs: f64) {
        let total = if total_secs > 0.1 { total_secs } else { 1.0 };
        let current = current_secs.clamp(0.0, total);
        self.b.progress_scale.set_range(0.0, total);
        self.b.progress_scale.set_value(current);
        self.b.time_current_label.set_label(&progress::format_timestamp(current));
        let tot_str = if total_secs > 0.1 { progress::format_timestamp(total) } else { "--:--".into() };
        self.b.time_total_label.set_label(&tot_str);
    }

    pub fn reset_progress(&self) {
        self.b.progress_scale.set_range(0.0, 100.0);
        self.b.progress_scale.set_value(0.0);
        self.b.time_current_label.set_label("00:00");
        self.b.time_total_label.set_label("00:00");
    }
}
