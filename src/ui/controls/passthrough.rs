use crate::audio::MicrophoneSource;
use gtk4 as gtk;
use gtk4::prelude::*;

pub fn create_passthrough_box(
    microphones: &[MicrophoneSource],
    selected_mic_node: Option<&str>,
    initial_passthrough: bool,
) -> (gtk::Box, gtk::DropDown, gtk::Switch) {
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

    (passthrough_box, mic_drop_down, passthrough_switch)
}
