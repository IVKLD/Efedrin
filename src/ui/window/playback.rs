use super::MainWindow;
use crate::sound::SoundItem;
use std::rc::Rc;
use std::time::Instant;

impl MainWindow {
    pub(super) fn play_sound(self: &Rc<Self>, item: &SoundItem) {
        if self.current_playing_id.borrow().as_deref() == Some(&item.id) {
            self.stop_all();
            return;
        }

        let mut item = item.clone();
        if item.duration_secs <= 0.0 {
            let probed = SoundItem::probe_duration(&item.path);
            if probed > 0.0 {
                item.update_duration(probed);
                self.library.borrow_mut().update_item_duration(&item.id, probed);
                self.sound_grid.update_sound_duration(&item.id, probed);
            }
        }

        let master = self.controls.master_volume();
        let vol_local = self.controls.local_volume() * master;
        let vol_mic = self.controls.mic_volume() * master;

        match self.audio_engine.play_sound(&item.path, vol_local, vol_mic, 0.0) {
            Ok(_) => {
                *self.current_playing_id.borrow_mut() = Some(item.id.clone());
                *self.current_playing_item.borrow_mut() = Some(item.clone());
                self.playback_start.set(Some(Instant::now()));
                self.playback_offset.set(0.0);
                self.controls.set_playback_progress(0.0, item.duration_secs);
                self.sound_grid.set_active_sound(Some(&item.id));
                self.status_label.set_label(&format!(
                    "Playing: {} (Sound: {:.0}%, Local: {:.0}%, Mic: {:.0}%)",
                    item.title, master * 100.0,
                    self.controls.local_volume() * 100.0,
                    self.controls.mic_volume() * 100.0
                ));
            }
            Err(e) => {
                self.stop_all();
                self.status_label.set_label(&format!("Playback error: {}", e));
            }
        }
    }

    pub(super) fn seek_playback(self: &Rc<Self>, target_secs: f64) {
        let current_item = self.current_playing_item.borrow().clone();
        if let Some(mut item) = current_item {
            if item.duration_secs <= 0.0 {
                let probed = SoundItem::probe_duration(&item.path);
                if probed > 0.0 {
                    item.update_duration(probed);
                    *self.current_playing_item.borrow_mut() = Some(item.clone());
                    self.library.borrow_mut().update_item_duration(&item.id, probed);
                    self.sound_grid.update_sound_duration(&item.id, probed);
                }
            }

            self.is_user_seeking.set(true);
            let master = self.controls.master_volume();
            let vol_local = self.controls.local_volume() * master;
            let vol_mic = self.controls.mic_volume() * master;
            let max_dur = if item.duration_secs > 0.0 { item.duration_secs } else { target_secs };
            let clamped = target_secs.clamp(0.0, max_dur);

            if let Ok(_) = self.audio_engine.play_sound(&item.path, vol_local, vol_mic, clamped) {
                self.playback_offset.set(clamped);
                self.playback_start.set(Some(Instant::now()));
                self.controls.set_playback_progress(clamped, item.duration_secs);
            }
            self.is_user_seeking.set(false);
        }
    }

    pub(super) fn stop_all(self: &Rc<Self>) {
        self.audio_engine.stop_all();
        *self.current_playing_id.borrow_mut() = None;
        *self.current_playing_item.borrow_mut() = None;
        self.playback_start.set(None);
        self.playback_offset.set(0.0);
        self.controls.reset_progress();
        self.sound_grid.set_active_sound(None);
        self.status_label.set_label("All playback stopped");
    }
}
