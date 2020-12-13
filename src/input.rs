mod event;
mod event_gate;
// use crate::{quicksilver_compat::Vector, Domain};
pub use event::*;
pub use event_gate::*;
// use nuts::{ActivityId, DomainState};

// pub(crate) struct InputState {
//     last_mouse_pos: Vector,
//     last_key: Option<Key>,
// }

// pub struct NewMousePos(pub Vector);

// impl InputState {
//     pub(crate) fn init() -> Self {
//         InputSetter::init();
//         Self {
//             last_mouse_pos: Vector::new(0, 0),
//             last_key: None,
//         }
//     }
//     pub fn last_mouse_pos(&self) -> Vector {
//         self.last_mouse_pos
//     }
//     pub fn last_key(&self) -> Option<Key> {
//         self.last_key
//     }
// }

// struct InputSetter;
// impl InputSetter {
//     fn init() {
//         let aid = nuts::new_domained_activity(InputSetter, &Domain::Frame);
//         aid.subscribe_domained(Self::update_mouse);
//     }
//     fn update_mouse(&mut self, domain: &mut DomainState, msg: &NewMousePos) {
//         let input = InputState::from_domain(domain);
//         input.last_mouse_pos = msg.0;
//     }
// }
