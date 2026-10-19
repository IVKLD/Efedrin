mod builder;
mod chips;
mod input;
mod suggestions;

use crate::sound::SoundItem;
use gtk4 as gtk;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TagEditorDialog;

struct TagEditorState {
    cur: RefCell<Vec<String>>,
    known: Vec<String>,
    save: Box<dyn Fn(Vec<String>)>,
    tags_flow: gtk::FlowBox,
    sug_flow: gtk::FlowBox,
    sug_header: gtk::Label,
}

impl TagEditorState {
    fn refresh(self: &Rc<Self>) {
        let tags = self.cur.borrow().clone();
        let st_rm = self.clone();
        chips::render_assigned_chips(&self.tags_flow, &tags, Rc::new(move |tag: &str| st_rm.remove_tag(tag)));
        let st_add = self.clone();
        suggestions::render_suggestions(&self.sug_flow, &self.sug_header, &self.known, &tags, Rc::new(move |tag: &str| st_add.add_tag(tag)));
    }

    fn add_tag(self: &Rc<Self>, tag: &str) {
        let mut t = self.cur.borrow_mut();
        if !t.iter().any(|existing| existing.eq_ignore_ascii_case(tag)) {
            t.push(tag.to_string());
            t.sort();
            t.dedup();
            drop(t);
            (self.save)(self.cur.borrow().clone());
            self.refresh();
        }
    }

    fn remove_tag(self: &Rc<Self>, tag: &str) {
        self.cur.borrow_mut().retain(|t| t != tag);
        (self.save)(self.cur.borrow().clone());
        self.refresh();
    }
}

impl TagEditorDialog {
    pub fn show<F>(parent: &impl IsA<gtk::Window>, item: &SoundItem, known: Vec<String>, on_save: F)
    where
        F: Fn(Vec<String>) + 'static,
    {
        let widgets = builder::create_dialog_window(parent, &item.title);
        let state = Rc::new(TagEditorState {
            cur: RefCell::new(item.tags.clone()),
            known,
            save: Box::new(on_save),
            tags_flow: widgets.tags_flow.clone(),
            sug_flow: widgets.suggestions_flow.clone(),
            sug_header: widgets.suggestions_header.clone(),
        });

        state.refresh();

        let st = state.clone();
        let (add_box, _) = input::create_input_section(move |clean_tag| st.add_tag(&clean_tag));

        widgets.content_box.append(&add_box);
        widgets.content_box.append(&widgets.suggestions_header);
        widgets.content_box.append(&widgets.suggestions_flow);
        widgets.window.present();
    }
}
