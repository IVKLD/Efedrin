use crate::audio::pipewire::{PipeWireManager, SINK_NODE_NAME};
use crate::error::{AppError, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

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

    fn clean_finished_processes(&self) {
        if let Ok(mut procs) = self.active_processes.lock() {
            procs.retain_mut(|child| match child.try_wait() {
                Ok(Some(_)) => false,
                Ok(None) => true,
                Err(_) => false,
            });
        }
    }

    pub fn play_sound(&self, sound_path: &Path, volume_local: f64, volume_mic: f64) -> Result<()> {
        if !sound_path.exists() {
            return Err(AppError::NotFound(format!(
                "Sound file not found: {}",
                sound_path.display()
            )));
        }

        {
            let mut pw = self
                .pipewire_manager
                .lock()
                .map_err(|e| AppError::Audio(format!("Mutex lock error: {}", e)))?;
            pw.start()?;
        }

        self.stop_all();
        self.clean_finished_processes();

        let path_str = sound_path
            .to_str()
            .ok_or_else(|| AppError::Audio("Invalid unicode in sound path".into()))?;

        let mut spawned = Vec::new();

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
                .map_err(|e| {
                    AppError::Audio(format!("Failed to play virtual mic stream: {}", e))
                })?;
            spawned.push(child_mic);
        }

        if let Ok(mut procs) = self.active_processes.lock() {
            procs.extend(spawned);
        }

        Ok(())
    }

    pub fn stop_all(&self) {
        if let Ok(mut procs) = self.active_processes.lock() {
            for mut child in procs.drain(..) {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.clean_finished_processes();
        if let Ok(procs) = self.active_processes.lock() {
            !procs.is_empty()
        } else {
            false
        }
    }

    pub fn set_mic_passthrough(&self, enabled: bool) -> Result<()> {
        let pw = self
            .pipewire_manager
            .lock()
            .map_err(|e| AppError::Audio(format!("Mutex lock error: {}", e)))?;
        pw.set_mic_passthrough(enabled)
    }

    #[allow(dead_code)]
    pub fn is_mic_passthrough_enabled(&self) -> bool {
        if let Ok(pw) = self.pipewire_manager.lock() {
            pw.is_mic_passthrough_enabled()
        } else {
            false
        }
    }

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
