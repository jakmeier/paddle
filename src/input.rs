mod event;
mod event_gate;
pub use event::*;
pub use event_gate::*;

pub(crate) struct InputState {
    last_key: Option<Key>,
}

impl InputState {
    pub(crate) fn new() -> Self {
        Self { last_key: None }
    }
    pub fn last_key(&self) -> Option<Key> {
        self.last_key
    }
}
