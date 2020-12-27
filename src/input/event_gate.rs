use crate::{
    js::PaddleJsContext, EventListenerType, FrameHandle, Key, KeyEvent, KeyEventType, MouseEvent,
    MouseEventType, Vector,
};
use div::DivHandle;
use nuts::{Activity, UncheckedActivityId};
use wasm_bindgen::prelude::wasm_bindgen;

pub(crate) struct RegisterEventListener {
    event_type: EventListenerType,
    div: DivHandle,
    activity: UncheckedActivityId,
}

/// Connection to events the browser forwards.
/// A single JS EventListener sits on the JS side of things and will call event_from_js(ID) with event IDs.
pub(crate) struct EventGate {
    js: PaddleJsContext,
}

#[wasm_bindgen(module = "/src/js/paddle.js")]
pub fn mouse_event_gate(activity_id: usize, event: MouseEventType, x: f32, y: f32) {
    let aid = UncheckedActivityId::forge_from_usize(activity_id);
    aid.private_message(MouseEvent(event, Vector::new(x, y)));
}
#[wasm_bindgen(module = "/src/js/paddle.js")]
pub fn keyboard_event_gate(activity_id: usize, event: KeyEventType, key: Key) {
    let aid = UncheckedActivityId::forge_from_usize(activity_id);
    aid.private_message(KeyEvent(event, key));
}

impl EventGate {
    pub(crate) fn init() {
        let gate = EventGate {
            js: PaddleJsContext::new(),
        };
        let aid = nuts::new_activity(gate);
        aid.private_channel(Self::register_event_listener);
    }
    pub fn listen<A: Activity>(frame: &FrameHandle<A>, event_type: EventListenerType) {
        nuts::send_to::<Self, _>(RegisterEventListener::new(frame, event_type));
    }
    fn register_event_listener(&mut self, msg: RegisterEventListener) {
        match msg.event_type {
            EventListenerType::Mouse(events) => {
                let html = msg.div.parent_element().unwrap();
                let aid = msg.activity.as_usize();
                for event in events {
                    self.js.register_mouse_event_listener(event, &html, aid);
                }
            }
            EventListenerType::KeyBoard(events) => {
                let aid = msg.activity.as_usize();
                for event in events {
                    self.js.register_keyboard_event_listener(event, aid);
                }
            }
        }
    }
}

impl RegisterEventListener {
    fn new<A: Activity>(frame: &FrameHandle<A>, event_type: EventListenerType) -> Self {
        Self {
            activity: frame.activity().into(),
            event_type,
            div: frame.div().clone(),
        }
    }
}
