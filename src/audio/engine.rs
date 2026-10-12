use crate::audio::pipewire::{PipeWireManager, SINK_NODE_NAME};
use crate::error::{AppError, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

pub const LOCAL_STREAM_NAME: &str = "efedrin_local_stream";
pub const MIC_STREAM_NAME: &str = "efedrin_mic_stream";

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

    pub fn play_sound(
        &self,
        sound_path: &Path,
        volume_local: f64,
        volume_mic: f64,
        start_seconds: f64,
    ) -> Result<()> {
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
            let mut cmd = Command::new("pw-play");
            cmd.arg(format!("--properties=node.name=\"{}\"", LOCAL_STREAM_NAME))
                .arg(format!("--volume={:.3}", clamped_vol_local));

            let child_local = if start_seconds > 0.1 {
                let ffmpeg = Command::new("ffmpeg")
                    .args([
                        "-ss",
                        &format!("{:.3}", start_seconds),
                        "-i",
                        path_str,
                        "-f",
                        "wav",
                        "-",
                    ])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|e| AppError::Audio(format!("Failed to spawn ffmpeg: {}", e)))?;

                let child = cmd
                    .arg("-")
                    .stdin(ffmpeg.stdout.unwrap())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|e| AppError::Audio(format!("Failed to play local stream: {}", e)))?;
                child
            } else {
                cmd.arg(path_str)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|e| AppError::Audio(format!("Failed to play local stream: {}", e)))?
            };
            spawned.push(child_local);
        }

        if volume_mic > 0.001 {
            let clamped_vol_mic = volume_mic.clamp(0.0, 2.0);
            let mut cmd = Command::new("pw-play");
            cmd.arg(format!("--target={}", SINK_NODE_NAME))
                .arg(format!("--properties=node.name=\"{}\"", MIC_STREAM_NAME))
                .arg(format!("--volume={:.3}", clamped_vol_mic));

            let child_mic = if start_seconds > 0.1 {
                let ffmpeg = Command::new("ffmpeg")
                    .args([
                        "-ss",
                        &format!("{:.3}", start_seconds),
                        "-i",
                        path_str,
                        "-f",
                        "wav",
                        "-",
                    ])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|e| AppError::Audio(format!("Failed to spawn ffmpeg: {}", e)))?;

                let child = cmd
                    .arg("-")
                    .stdin(ffmpeg.stdout.unwrap())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|e| {
                        AppError::Audio(format!("Failed to play virtual mic stream: {}", e))
                    })?;
                child
            } else {
                cmd.arg(path_str)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|e| {
                        AppError::Audio(format!("Failed to play virtual mic stream: {}", e))
                    })?
            };
            spawned.push(child_mic);
        }

        if let Ok(mut procs) = self.active_processes.lock() {
            procs.extend(spawned);
        }

        Ok(())
    }

    pub fn list_microphones(&self) -> Vec<crate::audio::pipewire::MicrophoneSource> {
        crate::audio::pipewire::PipeWireManager::list_microphones()
    }

    pub fn set_mic_passthrough(&self, enabled: bool, selected_mic: Option<&str>) -> Result<()> {
        let pw = self
            .pipewire_manager
            .lock()
            .map_err(|e| AppError::Audio(format!("Mutex lock error: {}", e)))?;
        pw.set_mic_passthrough(enabled, selected_mic)
    }

    fn find_node_ids_by_name(target_name: &str) -> Vec<String> {
        let output = match Command::new("pw-cli")
            .args(["list-objects", "Node"])
            .output()
        {
            Ok(out) => out,
            Err(_) => return Vec::new(),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut ids = Vec::new();
        let mut current_id = None;

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("id ") && trimmed.contains("type PipeWire:Interface:Node") {
                if let Some(id_part) = trimmed.strip_prefix("id ") {
                    if let Some(id_str) = id_part.split(',').next() {
                        current_id = Some(id_str.trim().to_string());
                    }
                }
            } else if trimmed.starts_with("node.name = ") {
                let name_val = trimmed.trim_start_matches("node.name = ").trim_matches('"');
                if name_val == target_name {
                    if let Some(id) = &current_id {
                        ids.push(id.clone());
                    }
                }
            }
        }

        ids
    }

    fn update_stream_volume(stream_name: &str, volume: f64) {
        let clamped = volume.clamp(0.0, 2.0);
        let ids = Self::find_node_ids_by_name(stream_name);
        for id in ids {
            let param_json = format!("{{ volume: {:.3} }}", clamped);
            let _ = Command::new("pw-cli")
                .args(["s", &id, "Props", &param_json])
                .output();
        }
    }

    pub fn set_local_volume(&self, volume: f64) {
        Self::update_stream_volume(LOCAL_STREAM_NAME, volume);
    }

    pub fn set_mic_volume(&self, volume: f64) {
        Self::update_stream_volume(MIC_STREAM_NAME, volume);
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
