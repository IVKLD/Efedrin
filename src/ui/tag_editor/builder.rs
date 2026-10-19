use gtk4 as gtk;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

pub struct TagEditorWidgets {
    pub window: adw::Window,
    pub content_box: gtk::Box,
    pub tags_flow: gtk::FlowBox,
    pub suggestions_header: gtk::Label,
    pub suggestions_flow: gtk::FlowBox,
}

fn section_header(title: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(title)
        .css_classes(["heading", "dim-label"])
        .halign(gtk::Align::Start)
        .build()
}

pub fn create_dialog_window(parent: &impl IsA<gtk::Window>, title: &str) -> TagEditorWidgets {
    let window = adw::Window::builder()
        .title(&format!("Tags — {}", title))
        .icon_name("efedrin")
        .transient_for(parent)
        .modal(true)
        .destroy_with_parent(true)
        .default_width(440)
        .default_height(380)
        .build();

    let header = adw::HeaderBar::new();
    header.set_show_end_title_buttons(false);
    let done_btn = gtk::Button::builder().label("Done").css_classes(["suggested-action"]).build();
    let win_clone = window.clone();
    done_btn.connect_clicked(move |_| win_clone.close());
    header.pack_end(&done_btn);

    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .margin_start(20).margin_end(20).margin_top(16).margin_bottom(20)
        .build();

    let track_title = gtk::Label::builder()
        .label(title)
        .css_classes(["title-3"])
        .halign(gtk::Align::Start)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .build();

    let tags_flow = gtk::FlowBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .column_spacing(8).row_spacing(8).valign(gtk::Align::Start)
        .build();

    let suggestions_header = section_header("Existing Library Tags");
    let suggestions_flow = gtk::FlowBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .column_spacing(6).row_spacing(6).valign(gtk::Align::Start)
        .build();

    content_box.append(&track_title);
    content_box.append(&section_header("Assigned Tags"));
    content_box.append(&tags_flow);
    content_box.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    content_box.append(&section_header("Create or Add Tag"));

    let scrolled = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&content_box).hexpand(true).vexpand(true)
        .build();

    let main_vbox = gtk::Box::builder().orientation(gtk::Orientation::Vertical).build();
    main_vbox.append(&header);
    main_vbox.append(&scrolled);
    window.set_content(Some(&main_vbox));

    TagEditorWidgets {
        window,
        content_box,
        tags_flow,
        suggestions_header,
        suggestions_flow,
    }
}
