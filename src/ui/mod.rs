pub mod controls;
pub mod header;
pub mod sound_card;
pub mod sound_grid;
pub mod tag_bar;
pub mod tag_editor;
pub mod window;

pub use window::MainWindow;

pub fn tag_color_class(tag: &str) -> &'static str {
    let mut hash: u32 = 5381;
    for byte in tag.to_lowercase().bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    match hash % 10 {
        0 => "tag-color-blue",
        1 => "tag-color-green",
        2 => "tag-color-orange",
        3 => "tag-color-purple",
        4 => "tag-color-red",
        5 => "tag-color-teal",
        6 => "tag-color-yellow",
        7 => "tag-color-pink",
        8 => "tag-color-indigo",
        _ => "tag-color-cyan",
    }
}
