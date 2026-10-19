pub mod constants;
mod playback;
mod pool;
mod volume;

pub use constants::*;

use crate::audio::pipewire::{MicrophoneSource, PipeWireManager};
use crate::error::{AppError, Result};
use pool::ProcessPool;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AudioEngine {
    pipewire: Arc<Mutex<PipeWireManager>>,
    pool: ProcessPool,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let mut pw = PipeWireManager::new();
        pw.start()?;
        Ok(Self {
            pipewire: Arc::new(Mutex::new(pw)),
            pool: ProcessPool::new(),
        })
    }

    pub fn play_sound(
        &self,
        path: &Path,
        vol_local: f64,
        vol_mic: f64,
        start_secs: f64,
    ) -> Result<()> {
        if !path.exists() {
            return Err(AppError::NotFound(format!("Sound file not found: {}", path.display())));
        }

        self.pipewire
            .lock()
            .map_err(|e| AppError::Audio(format!("Mutex error: {}", e)))?
            .start()?;

        self.stop_all();
        let spawned = playback::play_dual_stream(path, vol_local, vol_mic, start_secs)?;
        self.pool.set(spawned);
        Ok(())
    }

    pub fn list_microphones(&self) -> Vec<MicrophoneSource> {
        PipeWireManager::list_microphones()
    }

    pub fn set_mic_passthrough(&self, enabled: bool, mic: Option<&str>) -> Result<()> {
        let pw = self
            .pipewire
            .lock()
            .map_err(|e| AppError::Audio(format!("Mutex error: {}", e)))?;
        pw.set_mic_passthrough(enabled, mic)
    }

    pub fn set_local_volume(&self, vol: f64) {
        volume::update_stream_volume(LOCAL_STREAM_NAME, vol);
    }

    pub fn set_mic_volume(&self, vol: f64) {
        volume::update_stream_volume(MIC_STREAM_NAME, vol);
    }

    pub fn stop_all(&self) {
        self.pool.stop_all();
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.pool.is_playing()
    }

    pub fn shutdown(&self) {
        self.stop_all();
        if let Ok(mut pw) = self.pipewire.lock() {
            pw.stop();
        }
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.shutdown();
    }
}
