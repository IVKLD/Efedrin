use crate::error::{AppError, Result};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub const SINK_NODE_NAME: &str = "efedrin_soundpad";
pub const SOURCE_NODE_NAME: &str = "efedrin_mic";

/// Manages the lifecycle of PipeWire virtual devices (Sink + Virtual Microphone Source)
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

    /// Verifies that required PipeWire CLI tools exist on the system
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

    /// Checks if the virtual nodes are currently running and responsive
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

    /// Spawns pw-loopback to instantiate efedrin_soundpad (Sink) and efedrin_mic (Virtual Mic Source)
    pub fn start(&mut self) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        Self::check_prerequisites()?;

        // Clean up any stale instances with the same node names if they were left behind
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
            .map_err(|e| {
                AppError::PipeWire(format!("Failed to spawn pw-loopback: {}", e))
            })?;

        self.loopback_child = Some(child);

        // Give PipeWire daemon a short moment to register the nodes in the graph
        thread::sleep(Duration::from_millis(200));

        Ok(())
    }

    /// Stops the pw-loopback subprocess and cleans up virtual nodes
    pub fn stop(&mut self) {
        if let Some(mut child) = self.loopback_child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.mic_passthrough_enabled.store(false, Ordering::SeqCst);
    }

    /// Discovers hardware microphone capture ports in PipeWire
    fn discover_hardware_mic_ports() -> Vec<String> {
        let output = match Command::new("pw-link").arg("-o").output() {
            Ok(out) => out,
            Err(_) => return Vec::new(),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut mic_ports = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            // Look for alsa_input capture ports, excluding our own virtual microphone and monitor ports
            if trimmed.contains("alsa_input")
                && (trimmed.contains(":capture_") || trimmed.contains(":input_"))
                && !trimmed.contains("efedrin")
            {
                mic_ports.push(trimmed.to_string());
            }
        }

        mic_ports
    }

    /// Configures mixing of physical microphone into the virtual microphone
    pub fn set_mic_passthrough(&self, enable: bool) -> Result<()> {
        let mic_ports = Self::discover_hardware_mic_ports();
        let target_in_left = format!("{}:playback_FL", SINK_NODE_NAME);
        let target_in_right = format!("{}:playback_FR", SINK_NODE_NAME);

        for port in &mic_ports {
            if enable {
                // Link physical mic port to virtual soundpad sink input
                if port.ends_with("_FL") || port.ends_with("AUX0") || port.ends_with("1") {
                    let _ = Command::new("pw-link")
                        .args([port, &target_in_left])
                        .output();
                } else {
                    let _ = Command::new("pw-link")
                        .args([port, &target_in_right])
                        .output();
                }
            } else {
                // Disconnect physical mic from virtual soundpad sink
                let _ = Command::new("pw-link")
                    .args(["-d", port, &target_in_left])
                    .output();
                let _ = Command::new("pw-link")
                    .args(["-d", port, &target_in_right])
                    .output();
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
