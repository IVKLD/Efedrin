use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

pub fn render_suggestions<F>(
    flow: &gtk::FlowBox,
    header: &gtk::Label,
    all_known: &[String],
    current: &[String],
    on_add: Rc<F>,
) where
    F: Fn(&str) + 'static,
{
    while let Some(child) = flow.first_child() {
        flow.remove(&child);
    }

    let available: Vec<_> = all_known
        .iter()
        .filter(|t| !current.iter().any(|ct| ct.eq_ignore_ascii_case(t)))
        .collect();

    if available.is_empty() {
        header.set_visible(false);
        flow.set_visible(false);
        return;
    }

    header.set_visible(true);
    flow.set_visible(true);

    for tag in available {
        let btn = gtk::Button::builder()
            .label(&format!("+ {}", tag))
            .css_classes(["flat", "tag-pill", crate::ui::tag_color_class(tag)])
            .tooltip_text("Click to add this tag")
            .valign(gtk::Align::Center)
            .build();

        let tag_name = tag.clone();
        let cb = on_add.clone();
        btn.connect_clicked(move |_| cb(&tag_name));
        flow.insert(&btn, -1);
    }
}
