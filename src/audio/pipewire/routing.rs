use super::constants::SINK_NODE_NAME;
use super::devices::list_microphones;
use std::process::Command;

pub fn disconnect_soundpad_mic_inputs() {
    let Ok(output) = Command::new("pw-link").arg("-l").output() else {
        return;
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut current_dest = None;
    let target_prefix = format!("{}:playback_", SINK_NODE_NAME);

    for line in stdout.lines() {
        if !line.starts_with("  |") {
            let trimmed = line.trim();
            current_dest = if trimmed.starts_with(&target_prefix) {
                Some(trimmed.to_string())
            } else {
                None
            };
        } else if let Some(dest) = &current_dest {
            if let Some(src) = line.trim().strip_prefix("|<- ") {
                let src = src.trim();
                if !src.starts_with("efedrin_mic_stream:") {
                    let _ = Command::new("pw-link").args(["-d", src, dest]).output();
                }
            }
        }
    }
}

#[inline]
pub fn link_ports(src: &str, dest: &str) {
    let _ = Command::new("pw-link").args([src, dest]).output();
}

pub fn route_mic_passthrough(enable: bool, selected_mic_node: Option<&str>) {
    disconnect_soundpad_mic_inputs();

    if !enable {
        return;
    }

    let all_mics = list_microphones();
    let active_mic = selected_mic_node
        .and_then(|node| all_mics.iter().find(|m| m.node_name == node))
        .or_else(|| all_mics.first());

    let Some(mic) = active_mic else { return };
    let in_left = format!("{}:playback_FL", SINK_NODE_NAME);
    let in_right = format!("{}:playback_FR", SINK_NODE_NAME);

    if mic.ports.len() == 1 {
        link_ports(&mic.ports[0], &in_left);
        link_ports(&mic.ports[0], &in_right);
    } else {
        for port in &mic.ports {
            let is_left = port.ends_with("_FL")
                || port.ends_with("AUX0")
                || port.ends_with("_0")
                || port.ends_with(":capture_1");
            let is_right = port.ends_with("_FR")
                || port.ends_with("AUX1")
                || port.ends_with("_1")
                || port.ends_with(":capture_2");

            if is_left {
                link_ports(port, &in_left);
            } else if is_right {
                link_ports(port, &in_right);
            } else {
                link_ports(port, &in_left);
                link_ports(port, &in_right);
            }
        }
    }
}
