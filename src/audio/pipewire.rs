use crate::error::{AppError, Result};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub const SINK_NODE_NAME: &str = "efedrin_soundpad";
pub const SOURCE_NODE_NAME: &str = "efedrin_mic";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicrophoneSource {
    pub node_name: String,
    pub description: String,
    pub ports: Vec<String>,
}

pub struct PipeWireManager {
    loopback_child: Option<Child>,
    mic_passthrough_enabled: Arc<AtomicBool>,
}

impl PipeWireManager {
    pub fn new() -> Self {
        Self {
            loopback_child: None,
            mic_passthrough_enabled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn check_prerequisites() -> Result<()> {
        for bin in &["pw-loopback", "pw-link", "pw-play"] {
            if which::which(bin).is_err() {
                return Err(AppError::PipeWire(format!(
                    "Binary '{}' not found in PATH. Make sure PipeWire is installed.",
                    bin
                )));
            }
        }
        Ok(())
    }

    pub fn is_running(&mut self) -> bool {
        match self.loopback_child.as_mut().map(|c| c.try_wait()) {
            Some(Ok(None)) => true,
            _ => {
                self.loopback_child = None;
                false
            }
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        Self::check_prerequisites()?;

        let _ = Command::new("pkill")
            .args(["-f", "node.name=efedrin_soundpad"])
            .output();

        thread::sleep(Duration::from_millis(50));

        let capture_props = format!(
            "media.class=Audio/Sink node.name={} node.description=\"Efedrin Soundpad Sink\" resample.disable=false",
            SINK_NODE_NAME
        );
        let playback_props = format!(
            "media.class=Audio/Source node.name={} node.description=\"Efedrin Virtual Microphone\" resample.disable=false",
            SOURCE_NODE_NAME
        );

        let child = Command::new("pw-loopback")
            .arg(format!("--capture-props={}", capture_props))
            .arg(format!("--playback-props={}", playback_props))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| AppError::PipeWire(format!("Failed to spawn pw-loopback: {}", e)))?;

        self.loopback_child = Some(child);

        thread::sleep(Duration::from_millis(100));
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.loopback_child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        let _ = Command::new("pkill")
            .args(["-f", "node.name=efedrin_soundpad"])
            .output();
        self.mic_passthrough_enabled.store(false, Ordering::SeqCst);
    }

    fn fetch_node_descriptions() -> std::collections::HashMap<String, String> {
        let mut descriptions = std::collections::HashMap::new();
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
        let mut node_to_ports: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

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

        let descriptions = Self::fetch_node_descriptions();
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

    fn disconnect_soundpad_mic_inputs() {
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
    fn link_ports(src: &str, dest: &str) {
        let _ = Command::new("pw-link").args([src, dest]).output();
    }

    pub fn set_mic_passthrough(&self, enable: bool, selected_mic_node: Option<&str>) -> Result<()> {
        Self::disconnect_soundpad_mic_inputs();

        if enable {
            let all_mics = Self::list_microphones();
            let active_mic = selected_mic_node
                .and_then(|node| all_mics.iter().find(|m| m.node_name == node))
                .or_else(|| all_mics.first());

            if let Some(mic) = active_mic {
                let in_left = format!("{}:playback_FL", SINK_NODE_NAME);
                let in_right = format!("{}:playback_FR", SINK_NODE_NAME);

                if mic.ports.len() == 1 {
                    Self::link_ports(&mic.ports[0], &in_left);
                    Self::link_ports(&mic.ports[0], &in_right);
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
                            Self::link_ports(port, &in_left);
                        } else if is_right {
                            Self::link_ports(port, &in_right);
                        } else {
                            Self::link_ports(port, &in_left);
                            Self::link_ports(port, &in_right);
                        }
                    }
                }
            }
        }

        self.mic_passthrough_enabled.store(enable, Ordering::SeqCst);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_mic_passthrough_enabled(&self) -> bool {
        self.mic_passthrough_enabled.load(Ordering::SeqCst)
    }
}

impl Drop for PipeWireManager {
    fn drop(&mut self) {
        self.stop();
    }
}
