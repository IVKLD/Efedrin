use crate::sound::SoundItem;

pub enum LibraryEvent {
    DirectoryChanged,
    ScanComplete {
        generation: u64,
        items: Vec<SoundItem>,
        folder_display: String,
        is_initial: bool,
    },
    ScanError {
        generation: u64,
        error: String,
    },
}
