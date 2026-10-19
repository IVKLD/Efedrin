use std::process::Child;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct ProcessPool {
    procs: Arc<Mutex<Vec<Child>>>,
}

impl ProcessPool {
    pub fn new() -> Self {
        Self {
            procs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn set(&self, new_procs: Vec<Child>) {
        if let Ok(mut procs) = self.procs.lock() {
            *procs = new_procs;
        }
    }

    pub fn stop_all(&self) {
        if let Ok(mut procs) = self.procs.lock() {
            for mut child in procs.drain(..) {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        if let Ok(mut procs) = self.procs.lock() {
            procs.retain_mut(|c| matches!(c.try_wait(), Ok(None)));
            !procs.is_empty()
        } else {
            false
        }
    }
}
