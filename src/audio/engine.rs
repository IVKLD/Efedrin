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
            procs.retain_mut(|child| matches!(child.try_wait(), Ok(None)));
        }
    }

    fn prepare_sound_source(sound_path: &Path, start_seconds: f64) -> std::path::PathBuf {
        if start_seconds <= 0.05 {
            return sound_path.to_path_buf();
        }

        let ext = sound_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("mp3");
        let temp_path =
            std::env::temp_dir().join(format!("efedrin_seek_{}_{}.{}", std::process::id(), (start_seconds * 1000.0) as u64, ext));

        let run_ffmpeg = |copy_codec: bool| {
            let mut cmd = Command::new("ffmpeg");
            cmd.env_remove("LD_LIBRARY_PATH");
            cmd.arg("-ss")
                .arg(format!("{:.3}", start_seconds))
                .arg("-i")
                .arg(sound_path.to_str().unwrap_or_default());

            if copy_codec {
                cmd.args(["-c:a", "copy"]);
            }

            cmd.arg("-y")
                .arg(temp_path.to_str().unwrap_or_default())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
        };

        let success = match run_ffmpeg(true) {
            Ok(s) if s.success() && temp_path.metadata().map_or(false, |m| m.len() > 100) => true,
            _ => match run_ffmpeg(false) {
                Ok(s) => s.success() && temp_path.metadata().map_or(false, |m| m.len() > 100),
                Err(_) => false,
            },
        };

        if success {
            temp_path
        } else {
            sound_path.to_path_buf()
        }
    }

    fn spawn_stream(
        target: Option<&str>,
        stream_name: &str,
        volume: f64,
        file_path: &str,
    ) -> Result<Child> {
        let mut cmd = Command::new("pw-play");

        if let Some(target_node) = target {
            cmd.arg(format!("--target={}", target_node));
        }

        cmd.arg(format!(
            "--properties=application.name=\"efedrin_soundpad\" node.name=\"{}\"",
            stream_name
        ))
        .arg(format!("--volume={:.3}", volume.clamp(0.0, 10.0)))
        .arg(file_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

        cmd.spawn()
            .map_err(|e| AppError::Audio(format!("Failed to spawn stream '{}': {}", stream_name, e)))
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

        self.pipewire_manager
            .lock()
            .map_err(|e| AppError::Audio(format!("Mutex lock error: {}", e)))?
            .start()?;

        self.stop_all();

        let effective_path = Self::prepare_sound_source(sound_path, start_seconds);
        let path_str = effective_path
            .to_str()
            .ok_or_else(|| AppError::Audio("Invalid path string".into()))?;

        let mut spawned = Vec::with_capacity(2);

        if volume_local > 0.001 {
            spawned.push(Self::spawn_stream(None, LOCAL_STREAM_NAME, volume_local, path_str)?);
        }
        if volume_mic > 0.001 {
            spawned.push(Self::spawn_stream(Some(SINK_NODE_NAME), MIC_STREAM_NAME, volume_mic, path_str)?);
        }

        if let Ok(mut procs) = self.active_processes.lock() {
            *procs = spawned;
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

    fn update_stream_volume(stream_name: &str, volume: f64) {
        let param = format!("{{ volume: {:.3} }}", volume.clamp(0.0, 10.0));
        let _ = Command::new("pw-cli")
            .args(["s", stream_name, "Props", &param])
            .output();

        if let Ok(output) = Command::new("pw-cli").args(["list-objects", "Node"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut current_id = None;

            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("id ") && trimmed.contains("type PipeWire:Interface:Node") {
                    current_id = trimmed
                        .strip_prefix("id ")
                        .and_then(|id_part| id_part.split(',').next())
                        .map(str::trim);
                } else if trimmed.starts_with("node.name = ") {
                    let name = trimmed.trim_start_matches("node.name = ").trim_matches('"');
                    if name == stream_name {
                        if let Some(id) = current_id {
                            let _ = Command::new("pw-cli")
                                .args(["s", id, "Props", &param])
                                .output();
                        }
                    }
                }
            }
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
        self.active_processes
            .lock()
            .map(|procs| !procs.is_empty())
            .unwrap_or(false)
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
