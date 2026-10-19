use super::{SoundItem, SoundLibrary};
use std::path::{Path, PathBuf};

#[test]
fn test_sound_item_creation_and_duration_update() {
    let sample_path = Path::new("sample_sounds/coin.wav");
    if sample_path.exists() {
        let item = SoundItem::from_path(sample_path).expect("should parse sound item");
        assert_eq!(item.duration_secs, 0.0);
        assert_eq!(item.formatted_duration, "");
        assert_eq!(item.extension, "WAV");

        let mut library = SoundLibrary::new();
        library.set_items(vec![item.clone()]);
        library.update_item_duration(&item.id, 125.4);

        let updated = &library.items()[0];
        assert_eq!(updated.duration_secs, 125.4);
        assert_eq!(updated.formatted_duration, "02:05");
    }
}

#[test]
fn test_sound_library_filtering() {
    let mut library = SoundLibrary::new();
    let item1 = SoundItem {
        id: "1".into(),
        title: "victory fanfare".into(),
        path: PathBuf::from("/tmp/victory.mp3"),
        duration_secs: 10.0,
        formatted_duration: "00:10".into(),
        formatted_size: "1.0 MB".into(),
        extension: "MP3".into(),
        tags: vec![],
    };
    let item2 = SoundItem {
        id: "2".into(),
        title: "game over".into(),
        path: PathBuf::from("/tmp/game_over.wav"),
        duration_secs: 5.0,
        formatted_duration: "00:05".into(),
        formatted_size: "500 KB".into(),
        extension: "WAV".into(),
        tags: vec![],
    };

    library.set_items(vec![item1.clone(), item2.clone()]);

    let results = library.filter("victory");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "victory fanfare");

    let all = library.filter("");
    assert_eq!(all.len(), 2);

    let none = library.filter("nonexistent");
    assert_eq!(none.len(), 0);
}

#[test]
fn test_sound_library_tags_and_filtering() {
    let mut library = SoundLibrary::new();
    let item1 = SoundItem {
        id: "1".into(),
        title: "victory fanfare".into(),
        path: PathBuf::from("/tmp/victory.mp3"),
        duration_secs: 10.0,
        formatted_duration: "00:10".into(),
        formatted_size: "1.0 MB".into(),
        extension: "MP3".into(),
        tags: vec!["meme".into(), "music".into()],
    };
    let item2 = SoundItem {
        id: "2".into(),
        title: "game over".into(),
        path: PathBuf::from("/tmp/game_over.wav"),
        duration_secs: 5.0,
        formatted_duration: "00:05".into(),
        formatted_size: "500 KB".into(),
        extension: "WAV".into(),
        tags: vec!["meme".into(), "voice".into()],
    };

    library.set_items(vec![item1.clone(), item2.clone()]);
    assert_eq!(library.all_tags(), vec!["meme", "music", "voice"]);

    let meme_items = library.filter_with_tag("", Some("meme"));
    assert_eq!(meme_items.len(), 2);

    let victory_meme = library.filter_with_tag("victory", Some("meme"));
    assert_eq!(victory_meme.len(), 1);

    library.set_item_tags("1", vec!["sfx".into()]);
    assert_eq!(library.items()[0].tags, vec!["sfx"]);
}
