use super::constants::{LOCAL_STREAM_NAME, MIC_STREAM_NAME};
use crate::audio::pipewire::SINK_NODE_NAME;
use crate::error::{AppError, Result};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

fn run_ffmpeg_seek(src: &Path, dst: &Path, start: f64, copy: bool) -> bool {
    let mut cmd = Command::new("ffmpeg");
    cmd.env_remove("LD_LIBRARY_PATH");
    cmd.args(["-ss", &format!("{:.3}", start), "-i", src.to_str().unwrap_or_default()]);
    if copy {
        cmd.args(["-c:a", "copy"]);
    }
    cmd.args(["-y", dst.to_str().unwrap_or_default()])
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    cmd.status().map_or(false, |s| s.success()) && dst.metadata().map_or(false, |m| m.len() > 100)
}

pub fn prepare_sound_source(path: &Path, start: f64) -> PathBuf {
    if start <= 0.05 {
        return path.to_path_buf();
    }
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("mp3");
    let tmp = std::env::temp_dir().join(format!("efedrin_{}_{}.{}", std::process::id(), (start * 1000.0) as u64, ext));
    if run_ffmpeg_seek(path, &tmp, start, true) || run_ffmpeg_seek(path, &tmp, start, false) {
        tmp
    } else {
        path.to_path_buf()
    }
}

pub fn spawn_stream(target: Option<&str>, name: &str, vol: f64, file: &str) -> Result<Child> {
    let mut cmd = Command::new("pw-play");
    if let Some(target_node) = target {
        cmd.arg(format!("--target={}", target_node));
    }
    cmd.args([
        &format!("--properties=application.name=\"efedrin_soundpad\" node.name=\"{}\"", name),
        &format!("--volume={:.3}", vol.clamp(0.0, 10.0)),
        file,
    ])
    .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());

    cmd.spawn().map_err(|e| AppError::Audio(format!("Failed to spawn stream '{}': {}", name, e)))
}

pub fn play_dual_stream(path: &Path, v_loc: f64, v_mic: f64, start: f64) -> Result<Vec<Child>> {
    let src = prepare_sound_source(path, start);
    let p = src.to_str().ok_or_else(|| AppError::Audio("Invalid path".into()))?;
    let mut procs = Vec::with_capacity(2);
    if v_loc > 0.001 {
        procs.push(spawn_stream(None, LOCAL_STREAM_NAME, v_loc, p)?);
    }
    if v_mic > 0.001 {
        procs.push(spawn_stream(Some(SINK_NODE_NAME), MIC_STREAM_NAME, v_mic, p)?);
    }
    Ok(procs)
}
