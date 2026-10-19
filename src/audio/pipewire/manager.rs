use super::devices::{list_microphones, MicrophoneSource};
use super::loopback::{spawn_loopback, stop_loopback};
use super::routing::route_mic_passthrough;
use crate::error::Result;
use std::process::Child;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
        let child = spawn_loopback()?;
        self.loopback_child = Some(child);
        Ok(())
    }

    pub fn stop(&mut self) {
        stop_loopback(&mut self.loopback_child);
        self.mic_passthrough_enabled.store(false, Ordering::SeqCst);
    }

    pub fn list_microphones() -> Vec<MicrophoneSource> {
        list_microphones()
    }

    pub fn set_mic_passthrough(&self, enable: bool, selected_mic_node: Option<&str>) -> Result<()> {
        route_mic_passthrough(enable, selected_mic_node);
        self.mic_passthrough_enabled.store(enable, Ordering::SeqCst);
        Ok(())
    }
}

impl Drop for PipeWireManager {
    fn drop(&mut self) {
        self.stop();
    }
}
