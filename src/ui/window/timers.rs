use super::events::LibraryEvent;
use super::MainWindow;
use gtk4::glib;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub fn setup_unified_timer(
    main_window: &Rc<MainWindow>,
    scan_rx: Rc<RefCell<Receiver<LibraryEvent>>>,
) {
    let weak_window = Rc::downgrade(main_window);
    let engine = main_window.audio_engine.clone();
    let current_id = main_window.current_playing_id.clone();
    let current_item = main_window.current_playing_item.clone();
    let grid = main_window.sound_grid.clone();
    let status_lbl = main_window.status_label.clone();
    let controls = main_window.controls.clone();
    let start_cell = main_window.playback_start.clone();
    let offset_cell = main_window.playback_offset.clone();
    let seeking = main_window.is_user_seeking.clone();

    let mut stopped_ticks = 0u8;
    glib::timeout_add_local(Duration::from_millis(100), move || {
        let Some(this) = weak_window.upgrade() else { return glib::ControlFlow::Break };

        while let Ok(event) = scan_rx.borrow().try_recv() {
            this.handle_library_event(event);
        }

        if current_id.borrow().is_some() {
            if !engine.is_playing() {
                stopped_ticks += 1;
                if stopped_ticks >= 3 {
                    *current_id.borrow_mut() = None;
                    *current_item.borrow_mut() = None;
                    start_cell.set(None);
                    offset_cell.set(0.0);
                    grid.set_active_sound(None);
                    status_lbl.set_label("Ready");
                    controls.reset_progress();
                    stopped_ticks = 0;
                }
            } else {
                stopped_ticks = 0;
                if !seeking.get() {
                    if let (Some(started), Some(item)) = (start_cell.get(), current_item.borrow().as_ref()) {
                        let elapsed = offset_cell.get() + started.elapsed().as_secs_f64();
                        controls.set_playback_progress(elapsed, item.duration_secs);
                    }
                }
            }
        } else {
            stopped_ticks = 0;
        }
        glib::ControlFlow::Continue
    });
}
