use crate::quicksilver_compat::Vector;
use crate::{js::PaddleJsContext, FrameHandle, LeftClick};
use crate::{EventType, Key};
use div::PaneHandle;
use nuts::{Activity, ActivityId, DomainState, UncheckedActivityId};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;

pub(crate) struct RegisterEventListener {
    event_type: EventType,
    div: PaneHandle,
    activity: UncheckedActivityId,
}

/// Connection to events the browser forwards.
/// A single JS EventListener sits on the JS side of things and will call event_from_js(ID) with event IDs.
pub(crate) struct EventGate {
    js: PaddleJsContext,
}

// TODO
// #[wasm_bindgen]
// pub fn key_event(capsule_index: usize, event_type: Key) {
// let aid = UncheckedActivityId::forge_from_usize(activity_id);
// aid.private_message(KeyPressedd {
//     pos: Vector::new(x, y),
// });
// }

#[wasm_bindgen(module = "/src/js/paddle.js")]
pub fn mouse_event_gate(activity_id: usize, x: f32, y: f32) {
    let aid = UncheckedActivityId::forge_from_usize(activity_id);
    aid.private_message(LeftClick {
        pos: Vector::new(x, y),
    });
}

impl EventGate {
    pub(crate) fn init() {
        // let mouse_listener
        let gate = EventGate {
            js: PaddleJsContext::new(),
        };
        let aid = nuts::new_activity(gate);
        aid.private_channel(Self::register_event_listener);
    }
    pub fn listen<A: Activity>(frame: &FrameHandle<A>, event_type: EventType) {
        nuts::send_to::<Self, _>(RegisterEventListener::new(frame, event_type));
    }
    fn register_event_listener(&mut self, msg: RegisterEventListener) {
        match msg.event_type {
            EventType::Mouse(event) => {
                self.js.register_mouse_event_listener(
                    event as u32,
                    msg.div.parent_element().unwrap(),
                    msg.activity.as_usize(),
                );
            }
            _ => {
                // todo!()
            }
        }
    }
}

impl RegisterEventListener {
    fn new<A: Activity>(frame: &FrameHandle<A>, event_type: EventType) -> Self {
        Self {
            activity: frame.activity().into(),
            event_type,
            div: frame.div().unwrap().clone(),
        }
    }
}
