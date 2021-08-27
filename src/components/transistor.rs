use crate::components::wire::wiring;
use std::sync::{Arc, Mutex};

pub struct Transistor {
    pub base: wiring::Wire,
    pub collector: wiring::Wire,
    pub emitter: wiring::Wire,
}

impl Default for Transistor {
    fn default() -> Self {
        Transistor {
            base: Arc::new(Mutex::new(false)),
            collector: Arc::new(Mutex::new(false)),
            emitter: Arc::new(Mutex::new(false)),
        }
    }
}

impl Transistor {
    pub fn tick(&mut self) {
        let mut emitter_lock = self.emitter.lock().unwrap();
        if *self.base.lock().unwrap() {
            *emitter_lock = *self.collector.lock().unwrap();
        } else {
            *emitter_lock = false;
        }
    }
}
