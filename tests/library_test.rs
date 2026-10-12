use std::path::Path;

#[test]
fn test_sound_scanning() {
    let sample_dir = Path::new("sample_sounds");
    assert!(sample_dir.exists(), "sample_sounds directory must exist");

    let entries = std::fs::read_dir(sample_dir).expect("read dir");
    let mut count = 0;
    for entry in entries {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|s| s.to_str()) == Some("wav") {
            count += 1;
        }
    }
    assert!(count >= 3, "Expected at least 3 wav sounds");
}
