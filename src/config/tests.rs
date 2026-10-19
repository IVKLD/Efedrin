#[cfg(test)]
mod tests {
    use super::super::AppConfig;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_config_tag_helpers() {
        let mut config = AppConfig::default();
        config.sound_directory = Some(PathBuf::from("/home/user/sounds"));

        let file_in_dir = Path::new("/home/user/sounds/memes/laugh.mp3");
        let file_outside = Path::new("/var/sounds/beep.wav");

        assert_eq!(config.tag_key_for_path(file_in_dir), "memes/laugh.mp3");
        assert_eq!(config.tag_key_for_path(file_outside), "/var/sounds/beep.wav");

        assert!(config.get_track_tags(file_in_dir).is_empty());

        config.set_track_tags(
            file_in_dir,
            vec!["meme".into(), " voice ".into(), "meme".into(), "".into()],
        );

        let retrieved = config.get_track_tags(file_in_dir);
        assert_eq!(retrieved, vec!["meme", "voice"]);

        let serialized = serde_json::to_string(&config).expect("serialize config");
        let deserialized: AppConfig =
            serde_json::from_str(&serialized).expect("deserialize config");

        assert_eq!(deserialized.get_track_tags(file_in_dir), vec!["meme", "voice"]);

        config.set_track_tags(file_in_dir, Vec::new());
        assert!(config.get_track_tags(file_in_dir).is_empty());
        assert!(!config.track_tags.contains_key("memes/laugh.mp3"));
    }
}
