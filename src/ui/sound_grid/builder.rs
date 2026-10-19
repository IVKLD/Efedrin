use gtk4 as gtk;
use gtk4::prelude::*;
use libadwaita as adw;

pub struct GridWidgets {
    pub container: gtk::Box,
    pub flow_box: gtk::FlowBox,
    pub status_page: adw::StatusPage,
    pub scrolled_window: gtk::ScrolledWindow,
}

pub fn create_grid_widgets() -> GridWidgets {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();

    let flow_box = gtk::FlowBox::builder()
        .valign(gtk::Align::Start)
        .max_children_per_line(8)
        .min_children_per_line(2)
        .selection_mode(gtk::SelectionMode::None)
        .column_spacing(12)
        .row_spacing(12)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .child(&flow_box)
        .build();

    let status_page = adw::StatusPage::builder()
        .icon_name("audio-x-generic-symbolic")
        .title("No Sounds Loaded")
        .description("Select a folder with audio files to populate the soundboard.")
        .hexpand(true)
        .vexpand(true)
        .build();

    container.append(&status_page);
    container.append(&scrolled_window);
    scrolled_window.set_visible(false);

    GridWidgets {
        container,
        flow_box,
        status_page,
        scrolled_window,
    }
}
