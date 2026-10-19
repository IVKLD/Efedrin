use crate::sound::item::SoundItem;
use notify::{Event, EventKind};

pub fn is_relevant_event(event: &Event) -> bool {
    match event.kind {
        EventKind::Access(_) | EventKind::Other => false,
        _ => {
            if event.paths.is_empty() {
                return true;
            }
            event.paths.iter().any(|path| {
                if path.is_dir() {
                    return true;
                }
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    SoundItem::is_supported_extension(ext)
                } else {
                    true
                }
            })
        }
    }
}
