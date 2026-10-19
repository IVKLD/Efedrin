use super::MainWindow;
use crate::ui::header::Header;
use gtk4 as gtk;
use gtk4::prelude::*;
use std::rc::Rc;

pub fn setup_window_handlers(this: &Rc<MainWindow>, header: &Header) {
    let t_stop = this.clone();
    header.stop_button().connect_clicked(move |_| t_stop.stop_all());

    let key_controller = gtk::EventControllerKey::new();
    let t_key = this.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        if keyval == gtk4::gdk::Key::Escape || keyval == gtk4::gdk::Key::space {
            t_key.stop_all();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    this.window.add_controller(key_controller);

    let t_folder = this.clone();
    this.controls.folder_button().connect_clicked(move |_| t_folder.open_folder_dialog());

    let t_search = this.clone();
    this.controls.search_entry().connect_search_changed(move |_| t_search.apply_filter());

    let t_tag = this.clone();
    this.tag_bar.connect_tag_selected(move |_| t_tag.apply_filter());

    setup_audio_control_handlers(this);

    let t_seek = this.clone();
    this.controls.progress_scale().connect_change_value(move |_, _, target_val| {
        t_seek.seek_playback(target_val);
        glib::Propagation::Proceed
    });

    let (engine_close, cfg_close) = (this.audio_engine.clone(), this.config.clone());
    this.window.connect_close_request(move |_| {
        let _ = cfg_close.borrow().save();
        engine_close.shutdown();
        glib::Propagation::Proceed
    });
}

fn setup_audio_control_handlers(this: &Rc<MainWindow>) {
    let (cfg_m, eng_m, ctrl_m) = (this.config.clone(), this.audio_engine.clone(), this.controls.clone());
    this.controls.vol_master_scale().connect_value_changed(move |s| {
        let m = s.value();
        cfg_m.borrow_mut().volume_master = m;
        let _ = cfg_m.borrow().save();
        eng_m.set_local_volume(ctrl_m.local_volume() * m);
        eng_m.set_mic_volume(ctrl_m.mic_volume() * m);
    });

    let (cfg_l, eng_l, ctrl_l) = (this.config.clone(), this.audio_engine.clone(), this.controls.clone());
    this.controls.vol_local_scale().connect_value_changed(move |s| {
        let v = s.value();
        cfg_l.borrow_mut().volume_local = v;
        let _ = cfg_l.borrow().save();
        eng_l.set_local_volume(v * ctrl_l.master_volume());
    });

    let (cfg_mc, eng_mc, ctrl_mc) = (this.config.clone(), this.audio_engine.clone(), this.controls.clone());
    this.controls.vol_mic_scale().connect_value_changed(move |s| {
        let v = s.value();
        cfg_mc.borrow_mut().volume_mic = v;
        let _ = cfg_mc.borrow().save();
        eng_mc.set_mic_volume(v * ctrl_mc.master_volume());
    });

    let (eng_pt, cfg_pt, lbl_pt, ctrl_pt) = (this.audio_engine.clone(), this.config.clone(), this.status_label.clone(), this.controls.clone());
    this.controls.passthrough_switch().connect_state_set(move |_, state| {
        let mic = ctrl_pt.selected_microphone_node();
        let _ = eng_pt.set_mic_passthrough(state, mic.as_deref());
        cfg_pt.borrow_mut().mic_passthrough = state;
        let _ = cfg_pt.borrow().save();
        lbl_pt.set_label(if state { "Microphone passthrough enabled" } else { "Microphone passthrough disabled" });
        glib::Propagation::Proceed
    });

    let (eng_drop, cfg_drop, ctrl_drop) = (this.audio_engine.clone(), this.config.clone(), this.controls.clone());
    this.controls.mic_drop_down().connect_selected_notify(move |_| {
        let node = ctrl_drop.selected_microphone_node();
        cfg_drop.borrow_mut().selected_microphone = node.clone();
        let _ = cfg_drop.borrow().save();
        if ctrl_drop.passthrough_switch().is_active() {
            let _ = eng_drop.set_mic_passthrough(true, node.as_deref());
        }
    });
}
