use crate::error::{AppError, Result};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub const SINK_NODE_NAME: &str = "efedrin_soundpad";
pub const SOURCE_NODE_NAME: &str = "efedrin_mic";

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
        if which::which("pw-loopback").is_err() {
            return Err(AppError::PipeWire(
                "Binary 'pw-loopback' not found in PATH. Make sure PipeWire is installed.".into(),
            ));
        }
        if which::which("pw-link").is_err() {
            return Err(AppError::PipeWire(
                "Binary 'pw-link' not found in PATH. Make sure PipeWire is installed.".into(),
            ));
        }
        if which::which("pw-play").is_err() {
            return Err(AppError::PipeWire(
                "Binary 'pw-play' not found in PATH. Make sure PipeWire is installed.".into(),
            ));
        }
        Ok(())
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.loopback_child {
            match child.try_wait() {
                Ok(None) => true,
                _ => {
                    self.loopback_child = None;
                    false
                }
            }
        } else {
            false
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
            "media.class=Audio/Sink node.name={} node.description=\"Efedrin Soundpad Sink\"",
            SINK_NODE_NAME
        );
        let playback_props = format!(
            "media.class=Audio/Source node.name={} node.description=\"Efedrin Virtual Microphone\"",
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

        thread::sleep(Duration::from_millis(200));

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicrophoneSource {
    pub node_name: String,
    pub description: String,
    pub ports: Vec<String>,
}

impl PipeWireManager {
    pub fn list_microphones() -> Vec<MicrophoneSource> {
        let output = match Command::new("pw-link").arg("-o").output() {
            Ok(out) => out,
            Err(_) => return Vec::new(),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut node_to_ports: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.contains("efedrin") {
                continue;
            }
            if let Some((node, _port)) = trimmed.split_once(':') {
                if node.starts_with("alsa_input") || node.starts_with("easyeffects_source") {
                    node_to_ports
                        .entry(node.to_string())
                        .or_default()
                        .push(trimmed.to_string());
                }
            }
        }

        let desc_output = Command::new("pw-cli")
            .args(["list-objects", "Node"])
            .output()
            .ok();
        let mut descriptions = std::collections::HashMap::new();

        if let Some(out) = desc_output {
            let dump = String::from_utf8_lossy(&out.stdout);
            let mut curr_name = None;
            let mut curr_desc = None;

            for line in dump.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("id ") && trimmed.contains("type PipeWire:Interface:Node") {
                    if let (Some(n), Some(d)) = (curr_name.take(), curr_desc.take()) {
                        descriptions.insert(n, d);
                    }
                } else if trimmed.starts_with("node.name = ") {
                    curr_name = Some(
                        trimmed
                            .trim_start_matches("node.name = ")
                            .trim_matches('"')
                            .to_string(),
                    );
                } else if trimmed.starts_with("node.description = ") {
                    curr_desc = Some(
                        trimmed
                            .trim_start_matches("node.description = ")
                            .trim_matches('"')
                            .to_string(),
                    );
                }
            }
            if let (Some(n), Some(d)) = (curr_name.take(), curr_desc.take()) {
                descriptions.insert(n, d);
            }
        }

        let mut result = Vec::new();
        for (node_name, ports) in node_to_ports {
            let description = descriptions
                .get(&node_name)
                .cloned()
                .unwrap_or_else(|| node_name.clone());
            result.push(MicrophoneSource {
                node_name,
                description,
                ports,
            });
        }

        result.sort_by(|a, b| a.description.cmp(&b.description));
        result
    }

    fn disconnect_soundpad_mic_inputs() {
        if let Ok(output) = Command::new("pw-link").arg("-l").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut current_port: Option<String> = None;

            for line in stdout.lines() {
                if !line.starts_with("  |") {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        current_port = Some(trimmed.to_string());
                    }
                } else if let Some(dest) = &current_port {
                    if dest.starts_with(&format!("{}:playback_", SINK_NODE_NAME)) {
                        let trimmed = line.trim();
                        if let Some(src) = trimmed.strip_prefix("|<- ") {
                            let src = src.trim();
                            if !src.starts_with("efedrin_mic_stream:") {
                                let _ = Command::new("pw-link").args(["-d", src, dest]).output();
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn set_mic_passthrough(&self, enable: bool, selected_mic_node: Option<&str>) -> Result<()> {
        Self::disconnect_soundpad_mic_inputs();

        if enable {
            let all_mics = Self::list_microphones();
            let active_mic = if let Some(node) = selected_mic_node {
                all_mics.iter().find(|m| m.node_name == node)
            } else {
                all_mics.first()
            };

            let target_in_left = format!("{}:playback_FL", SINK_NODE_NAME);
            let target_in_right = format!("{}:playback_FR", SINK_NODE_NAME);

            if let Some(mic) = active_mic {
                if mic.ports.len() == 1 {
                    let port = &mic.ports[0];
                    let _ = Command::new("pw-link")
                        .args([port, &target_in_left])
                        .output();
                    let _ = Command::new("pw-link")
                        .args([port, &target_in_right])
                        .output();
                } else {
                    for port in &mic.ports {
                        if port.ends_with("_FL") || port.ends_with("AUX0") || port.ends_with("1") {
                            let _ = Command::new("pw-link")
                                .args([port, &target_in_left])
                                .output();
                        } else {
                            let _ = Command::new("pw-link")
                                .args([port, &target_in_right])
                                .output();
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
