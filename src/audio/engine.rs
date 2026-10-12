use crate::audio::pipewire::{PipeWireManager, SINK_NODE_NAME};
use crate::error::{AppError, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

/// Dual-routing Audio Engine directing sound to local playback and virtual microphone
#[derive(Clone)]
pub struct AudioEngine {
    pipewire_manager: Arc<Mutex<PipeWireManager>>,
    active_processes: Arc<Mutex<Vec<Child>>>,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let mut pw = PipeWireManager::new();
        pw.start()?;

        Ok(Self {
            pipewire_manager: Arc::new(Mutex::new(pw)),
            active_processes: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Prunes finished child processes from the tracking list
    fn clean_finished_processes(&self) {
        if let Ok(mut procs) = self.active_processes.lock() {
            procs.retain_mut(|child| match child.try_wait() {
                Ok(Some(_)) => false, // Process finished, remove from list
                Ok(None) => true,     // Process still running, keep
                Err(_) => false,
            });
        }
    }

    /// Plays a sound file simultaneously to user headphones and PipeWire virtual microphone
    pub fn play_sound(&self, sound_path: &Path, volume_local: f64, volume_mic: f64) -> Result<()> {
        if !sound_path.exists() {
            return Err(AppError::NotFound(format!(
                "Sound file not found: {}",
                sound_path.display()
            )));
        }

        // Ensure PipeWire virtual nodes are active
        {
            let mut pw = self
                .pipewire_manager
                .lock()
                .map_err(|e| AppError::Audio(format!("Mutex lock error: {}", e)))?;
            pw.start()?;
        }

        self.clean_finished_processes();

        let path_str = sound_path
            .to_str()
            .ok_or_else(|| AppError::Audio("Invalid unicode in sound path".into()))?;

        let mut spawned = Vec::new();

        // 1. Local Playback (Headphones / Default Audio Output)
        if volume_local > 0.001 {
            let clamped_vol_local = volume_local.clamp(0.0, 2.0);
            let child_local = Command::new("pw-play")
                .arg(format!("--volume={:.3}", clamped_vol_local))
                .arg(path_str)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| AppError::Audio(format!("Failed to play local stream: {}", e)))?;
            spawned.push(child_local);
        }

        // 2. Virtual Microphone Playback (Target: efedrin_soundpad)
        if volume_mic > 0.001 {
            let clamped_vol_mic = volume_mic.clamp(0.0, 2.0);
            let child_mic = Command::new("pw-play")
                .arg(format!("--target={}", SINK_NODE_NAME))
                .arg(format!("--volume={:.3}", clamped_vol_mic))
                .arg(path_str)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| AppError::Audio(format!("Failed to play virtual mic stream: {}", e)))?;
            spawned.push(child_mic);
        }

        if let Ok(mut procs) = self.active_processes.lock() {
            procs.extend(spawned);
        }

        Ok(())
    }

    /// Immediately stops all active sounds playing on local or virtual devices
    pub fn stop_all(&self) {
        if let Ok(mut procs) = self.active_processes.lock() {
            for mut child in procs.drain(..) {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    /// Checks if any sound stream is currently active
    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.clean_finished_processes();
        if let Ok(procs) = self.active_processes.lock() {
            !procs.is_empty()
        } else {
            false
        }
    }

    /// Toggles microphone mixing into the virtual microphone
    pub fn set_mic_passthrough(&self, enabled: bool) -> Result<()> {
        let pw = self
            .pipewire_manager
            .lock()
            .map_err(|e| AppError::Audio(format!("Mutex lock error: {}", e)))?;
        pw.set_mic_passthrough(enabled)
    }

    /// Queries whether mic passthrough is currently enabled
    #[allow(dead_code)]
    pub fn is_mic_passthrough_enabled(&self) -> bool {
        if let Ok(pw) = self.pipewire_manager.lock() {
            pw.is_mic_passthrough_enabled()
        } else {
            false
        }
    }

    /// Shuts down PipeWire virtual nodes on application exit
    pub fn shutdown(&self) {
        self.stop_all();
        if let Ok(mut pw) = self.pipewire_manager.lock() {
            pw.stop();
        }
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.shutdown();
    }
}
