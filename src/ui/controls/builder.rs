use super::{folder, passthrough, progress, volume};
use crate::audio::MicrophoneSource;
use gtk4 as gtk;
use gtk4::prelude::*;

pub struct BuiltControls {
    pub container: gtk::Box,
    pub folder_button: gtk::Button,
    pub folder_label: gtk::Label,
    pub search_entry: gtk::SearchEntry,
    pub vol_master_scale: gtk::Scale,
    pub vol_local_scale: gtk::Scale,
    pub vol_mic_scale: gtk::Scale,
    pub passthrough_switch: gtk::Switch,
    pub mic_drop_down: gtk::DropDown,
    pub progress_scale: gtk::Scale,
    pub time_current_label: gtk::Label,
    pub time_total_label: gtk::Label,
}

pub fn build_controls(
    v_master: f64,
    v_local: f64,
    v_mic: f64,
    pt: bool,
    mics: &[MicrophoneSource],
    selected_mic: Option<&str>,
) -> BuiltControls {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_start(16).margin_end(16).margin_top(12).margin_bottom(8)
        .build();

    let (folder_button, folder_label) = folder::create_folder_button();
    let search_entry = gtk::SearchEntry::builder().placeholder_text("Search sounds...").hexpand(true).build();
    let (pt_box, mic_drop_down, passthrough_switch) = passthrough::create_passthrough_box(mics, selected_mic, pt);

    let row1 = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(12).build();
    row1.append(&folder_button);
    row1.append(&search_entry);
    row1.append(&pt_box);

    let (m_box, vol_master_scale) = volume::create_volume_channel("media-playback-start-symbolic", "Sound Vol:", v_master);
    let (l_box, vol_local_scale) = volume::create_volume_channel("audio-headphones-symbolic", "Headphones:", v_local);
    let (mc_box, vol_mic_scale) = volume::create_volume_channel("audio-volume-high-symbolic", "Virtual Mic:", v_mic);

    let row2 = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(18).homogeneous(true).build();
    row2.append(&m_box);
    row2.append(&l_box);
    row2.append(&mc_box);

    let pw = progress::create_progress_box();
    container.append(&row1);
    container.append(&row2);
    container.append(&pw.progress_box);

    BuiltControls {
        container, folder_button, folder_label, search_entry,
        vol_master_scale, vol_local_scale, vol_mic_scale, passthrough_switch,
        mic_drop_down, progress_scale: pw.progress_scale,
        time_current_label: pw.time_current_label, time_total_label: pw.time_total_label,
    }
}
