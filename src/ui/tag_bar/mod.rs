mod builder;
mod buttons;

use gtk4 as gtk;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TagBar {
    scrolled: gtk::ScrolledWindow,
    buttons_box: gtk::Box,
    active_tag: Rc<RefCell<Option<String>>>,
    on_tag_selected: Rc<RefCell<Option<Rc<dyn Fn(Option<String>)>>>>,
    current_counts: Rc<RefCell<Vec<(String, usize)>>>,
    current_total: Rc<RefCell<usize>>,
}

impl TagBar {
    pub fn new() -> Self {
        let widgets = builder::create_tag_bar_widgets();
        Self {
            scrolled: widgets.scrolled,
            buttons_box: widgets.buttons_box,
            active_tag: Rc::new(RefCell::new(None)),
            on_tag_selected: Rc::new(RefCell::new(None)),
            current_counts: Rc::new(RefCell::new(Vec::new())),
            current_total: Rc::new(RefCell::new(0)),
        }
    }

    pub fn widget(&self) -> &gtk::ScrolledWindow {
        &self.scrolled
    }

    pub fn active_tag(&self) -> Option<String> {
        self.active_tag.borrow().clone()
    }

    pub fn connect_tag_selected<F: Fn(Option<String>) + 'static>(&self, f: F) {
        *self.on_tag_selected.borrow_mut() = Some(Rc::new(f));
    }

    pub fn update_tags(&self, total_sounds: usize, tag_counts: &[(String, usize)]) {
        *self.current_total.borrow_mut() = total_sounds;
        *self.current_counts.borrow_mut() = tag_counts.to_vec();

        while let Some(child) = self.buttons_box.first_child() {
            self.buttons_box.remove(&child);
        }

        if total_sounds == 0 {
            self.scrolled.set_visible(false);
            return;
        }

        self.scrolled.set_visible(true);

        let is_all_active = self.active_tag.borrow().is_none();
        let all_btn = buttons::create_all_button(
            total_sounds,
            is_all_active,
            self.active_tag.clone(),
            self.buttons_box.clone(),
            self.on_tag_selected.clone(),
        );
        self.buttons_box.append(&all_btn);

        for (tag, count) in tag_counts {
            let is_active = self.active_tag.borrow().as_deref() == Some(tag.as_str());
            let btn = buttons::create_tag_button(
                tag,
                *count,
                is_active,
                self.active_tag.clone(),
                self.buttons_box.clone(),
                self.on_tag_selected.clone(),
            );
            self.buttons_box.append(&btn);
        }
    }

    pub fn select_tag(&self, tag: Option<&str>) {
        *self.active_tag.borrow_mut() = tag.map(|s| s.to_string());
        buttons::refresh_active_styles(&self.buttons_box, tag);

        if let Some(cb) = self.on_tag_selected.borrow().as_ref() {
            cb(tag.map(|s| s.to_string()));
        }
    }
}
