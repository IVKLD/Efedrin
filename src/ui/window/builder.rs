use crate::ui::controls::ControlsBar;
use crate::ui::header::Header;
use crate::ui::sound_grid::SoundGrid;
use crate::ui::tag_bar::TagBar;
use gtk4 as gtk;
use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;
use std::rc::Rc;

pub struct WindowWidgets {
    pub window: adw::ApplicationWindow,
    pub header: Header,
    pub status_label: gtk::Label,
    pub count_label: gtk::Label,
}

pub fn build_main_window(
    app: &adw::Application,
    controls: &Rc<ControlsBar>,
    tag_bar: &Rc<TagBar>,
    sound_grid: &Rc<SoundGrid>,
) -> WindowWidgets {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Efedrin Soundpad")
        .icon_name("efedrin")
        .default_width(860)
        .default_height(620)
        .build();

    let header = Header::new();
    let main_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).build();
    main_box.append(header.widget());

    let sep1 = gtk::Separator::new(gtk::Orientation::Horizontal);
    let sep2 = gtk::Separator::new(gtk::Orientation::Horizontal);

    let status_bar = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(12)
        .margin_start(16).margin_end(16).margin_top(6).margin_bottom(6)
        .build();

    let status_label = gtk::Label::builder()
        .label("Ready")
        .css_classes(["caption"])
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();

    let count_label = gtk::Label::builder()
        .label("0 sounds loaded")
        .css_classes(["caption", "dim-label"])
        .halign(gtk::Align::End)
        .build();

    status_bar.append(&status_label);
    status_bar.append(&count_label);

    main_box.append(controls.widget());
    main_box.append(tag_bar.widget());
    main_box.append(&sep1);
    main_box.append(sound_grid.widget());
    main_box.append(&sep2);
    main_box.append(&status_bar);

    window.set_content(Some(&main_box));

    WindowWidgets {
        window,
        header,
        status_label,
        count_label,
    }
}
