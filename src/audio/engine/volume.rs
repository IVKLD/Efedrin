use std::process::Command;

pub fn update_stream_volume(stream_name: &str, volume: f64) {
    let param = format!("{{ volume: {:.3} }}", volume.clamp(0.0, 10.0));
    let _ = Command::new("pw-cli")
        .args(["s", stream_name, "Props", &param])
        .output();
}
