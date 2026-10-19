use super::MainWindow;
use crate::sound::SoundItem;
use crate::ui::tag_editor::TagEditorDialog;
use gtk4::prelude::*;
use std::rc::Rc;

impl MainWindow {
    pub(super) fn apply_filter(self: &Rc<Self>) {
        let query = self.controls.search_entry().text().to_string();
        let tag = self.tag_bar.active_tag();
        let filtered = self.library.borrow().filter_with_tag(&query, tag.as_deref());
        self.render_sound_cards(&filtered);
    }

    pub(super) fn open_tag_editor(self: &Rc<Self>, item: &SoundItem) {
        let all_tags = self.library.borrow().all_tags();
        let this = self.clone();
        let item_path = item.path.clone();
        let item_id = item.id.clone();

        TagEditorDialog::show(
            &self.window,
            item,
            all_tags,
            move |new_tags| {
                this.config.borrow_mut().set_track_tags(&item_path, new_tags.clone());
                let _ = this.config.borrow().save();
                this.library.borrow_mut().set_item_tags(&item_id, new_tags);

                let total = this.library.borrow().items().len();
                let tag_counts = this.library.borrow().tag_counts();
                this.tag_bar.update_tags(total, &tag_counts);
                this.apply_filter();
            },
        );
    }

    pub(super) fn render_sound_cards(self: &Rc<Self>, items: &[SoundItem]) {
        let (this_p, this_e, this_t) = (self.clone(), self.clone(), self.clone());
        let on_play = Rc::new(move |item: &SoundItem| this_p.play_sound(item));
        let on_edit = Rc::new(move |item: &SoundItem| this_e.open_tag_editor(item));
        let on_tag = Rc::new(move |tag: &str| {
            this_t.tag_bar.select_tag(Some(tag));
            this_t.apply_filter();
        });

        self.sound_grid.populate(
            items,
            on_play,
            on_edit,
            on_tag,
            self.current_playing_id.borrow().as_deref(),
        );
    }
}
