use gtk4 as gtk;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn refresh_active_styles(buttons_box: &gtk::Box, active_tag: Option<&str>) {
    let mut child = buttons_box.first_child();
    let mut is_first = true;
    while let Some(w) = child {
        let is_active = if is_first {
            is_first = false;
            active_tag.is_none()
        } else {
            active_tag == Some(w.widget_name().as_str())
        };
        if is_active {
            w.add_css_class("badge-active");
        } else {
            w.remove_css_class("badge-active");
        }
        child = w.next_sibling();
    }
}

pub fn create_all_button(
    total: usize,
    active: bool,
    active_tag: Rc<RefCell<Option<String>>>,
    buttons_box: gtk::Box,
    cb: Rc<RefCell<Option<Rc<dyn Fn(Option<String>)>>>>,
) -> gtk::Button {
    let btn = gtk::Button::builder()
        .label(&format!("All ({})", total))
        .css_classes(["badge", "pill"])
        .tooltip_text("Show all sounds")
        .build();
    if active {
        btn.add_css_class("badge-active");
    }
    btn.connect_clicked(move |_| {
        *active_tag.borrow_mut() = None;
        refresh_active_styles(&buttons_box, None);
        if let Some(f) = cb.borrow().as_ref() {
            f(None);
        }
    });
    btn
}

pub fn create_tag_button(
    tag: &str,
    count: usize,
    active: bool,
    active_tag: Rc<RefCell<Option<String>>>,
    buttons_box: gtk::Box,
    cb: Rc<RefCell<Option<Rc<dyn Fn(Option<String>)>>>>,
) -> gtk::Button {
    let btn = gtk::Button::builder()
        .label(&format!("#{} ({})", tag, count))
        .css_classes(["badge", "tag-pill", crate::ui::tag_color_class(tag)])
        .tooltip_text(&format!("Filter by tag '{}'", tag))
        .build();
    btn.set_widget_name(tag);
    if active {
        btn.add_css_class("badge-active");
    }
    let tag_name = tag.to_string();
    btn.connect_clicked(move |_| {
        let new_active = {
            let mut cur = active_tag.borrow_mut();
            if cur.as_deref() == Some(&tag_name) {
                *cur = None;
                None
            } else {
                *cur = Some(tag_name.clone());
                Some(tag_name.clone())
            }
        };
        refresh_active_styles(&buttons_box, new_active.as_deref());
        if let Some(f) = cb.borrow().as_ref() {
            f(new_active);
        }
    });
    btn
}
