use super::constants::{SINK_NODE_NAME, SOURCE_NODE_NAME};
use crate::error::{AppError, Result};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

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

pub fn spawn_loopback() -> Result<Child> {
    check_prerequisites()?;
    let _ = Command::new("pkill")
        .args(["-f", "node.name=efedrin_soundpad"])
        .output();
    thread::sleep(Duration::from_millis(50));

    let capture = format!(
        "media.class=Audio/Sink node.name={} node.description=\"Efedrin Soundpad Sink\" resample.disable=false",
        SINK_NODE_NAME
    );
    let playback = format!(
        "media.class=Audio/Source node.name={} node.description=\"Efedrin Virtual Microphone\" resample.disable=false",
        SOURCE_NODE_NAME
    );

    let child = Command::new("pw-loopback")
        .arg(format!("--capture-props={}", capture))
        .arg(format!("--playback-props={}", playback))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| AppError::PipeWire(format!("Failed to spawn pw-loopback: {}", e)))?;

    thread::sleep(Duration::from_millis(100));
    Ok(child)
}

pub fn stop_loopback(child: &mut Option<Child>) {
    if let Some(mut c) = child.take() {
        let _ = c.kill();
        let _ = c.wait();
    }
    let _ = Command::new("pkill")
        .args(["-f", "node.name=efedrin_soundpad"])
        .output();
}
