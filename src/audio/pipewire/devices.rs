use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicrophoneSource {
    pub node_name: String,
    pub description: String,
    pub ports: Vec<String>,
}

pub fn fetch_node_descriptions() -> HashMap<String, String> {
    let mut descriptions = HashMap::new();
    let Ok(out) = Command::new("pw-cli").args(["list-objects", "Node"]).output() else {
        return descriptions;
    };

    let dump = String::from_utf8_lossy(&out.stdout);
    let mut curr_name = None;
    let mut curr_desc = None;

    for line in dump.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("id ") && trimmed.contains("type PipeWire:Interface:Node") {
            if let (Some(n), Some(d)) = (curr_name.take(), curr_desc.take()) {
                descriptions.insert(n, d);
            }
        } else if let Some(val) = trimmed.strip_prefix("node.name = ") {
            curr_name = Some(val.trim_matches('"').to_string());
        } else if let Some(val) = trimmed.strip_prefix("node.description = ") {
            curr_desc = Some(val.trim_matches('"').to_string());
        }
    }

    if let (Some(n), Some(d)) = (curr_name, curr_desc) {
        descriptions.insert(n, d);
    }

    descriptions
}

pub fn list_microphones() -> Vec<MicrophoneSource> {
    let Ok(output) = Command::new("pw-link").arg("-o").output() else {
        return Vec::new();
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut node_to_ports: HashMap<String, Vec<String>> = HashMap::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.contains("efedrin") {
            continue;
        }
        if let Some((node, _)) = trimmed.split_once(':') {
            if node.starts_with("alsa_input") || node.starts_with("easyeffects_source") {
                node_to_ports
                    .entry(node.to_string())
                    .or_default()
                    .push(trimmed.to_string());
            }
        }
    }

    let descriptions = fetch_node_descriptions();
    let mut result: Vec<_> = node_to_ports
        .into_iter()
        .map(|(node_name, ports)| {
            let description = descriptions
                .get(&node_name)
                .cloned()
                .unwrap_or_else(|| node_name.clone());
            MicrophoneSource {
                node_name,
                description,
                ports,
            }
        })
        .collect();

    result.sort_by(|a, b| a.description.cmp(&b.description));
    result
}
