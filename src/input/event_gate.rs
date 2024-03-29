use super::browser_pointer_events::TouchEventType;
use super::event::*;
use super::{browser_pointer_events::*, keys::Key};
use crate::js::{
    ClickEventGateFnType, KeyboardEventGateFnType, MouseEventGateFnType, PointerEventGateFnType,
    TouchEventGateFnType,
};
use crate::{js::PaddleJsContext, FrameHandle, Vector};
use div::DivHandle;
use nuts::{Activity, UncheckedActivityId};
use std::any::Any;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::Closure;

pub(crate) struct RegisterEventListener {
    event_type: EventListenerType,
    div: DivHandle,
    activity: UncheckedActivityId,
}

pub enum EventListenerType {
    Click,
    Keyboard,
    Mouse,
    Touch,
    BrowserPointer,
    // Possible extension for low-level events:
    // BrowserEvent
}

/// Connection to events the browser forwards.
/// A single JS EventListener sits on the JS side of things and will call event_from_js(ID) with event IDs.
pub(crate) struct EventGate {
    js: PaddleJsContext,
    _closure_guards: Vec<Box<dyn Any>>,
}

/// handler injected to PaddleJsContext
fn click_event_gate(activity_id: usize, event: ClickEventType, x: f32, y: f32) {
    let aid = UncheckedActivityId::forge_from_usize(activity_id);
    aid.private_message(PointerEvent(event.into(), Vector::new(x, y)));
}

/// handler injected to PaddleJsContext
fn mouse_event_gate(activity_id: usize, event: MouseEventType, x: f32, y: f32) {
    let aid = UncheckedActivityId::forge_from_usize(activity_id);
    aid.private_message(PointerEvent(event.into(), Vector::new(x, y)));
}

/// handler injected to PaddleJsContext
fn touch_event_gate(activity_id: usize, event: TouchEventType, x: f32, y: f32) {
    let aid = UncheckedActivityId::forge_from_usize(activity_id);
    aid.private_message(PointerEvent(event.into(), Vector::new(x, y)));
}

/// handler injected to PaddleJsContext
fn pointer_event_gate(activity_id: usize, event: BrowserPointerEventType, x: f32, y: f32) {
    let aid = UncheckedActivityId::forge_from_usize(activity_id);
    aid.private_message(PointerEvent(event.into(), Vector::new(x, y)));
}

/// handler injected to PaddleJsContext
fn keyboard_event_gate(activity_id: usize, event: KeyEventType, key: Key) {
    let aid = UncheckedActivityId::forge_from_usize(activity_id);
    aid.private_message(KeyEvent(event, key));
}

impl EventGate {
    pub(crate) fn init() {
        let click = Closure::wrap(Box::new(click_event_gate) as Box<ClickEventGateFnType>);
        let mouse = Closure::wrap(Box::new(mouse_event_gate) as Box<MouseEventGateFnType>);
        let key = Closure::wrap(Box::new(keyboard_event_gate) as Box<KeyboardEventGateFnType>);
        let pointer = Closure::wrap(Box::new(pointer_event_gate) as Box<PointerEventGateFnType>);
        let touch = Closure::wrap(Box::new(touch_event_gate) as Box<TouchEventGateFnType>);

        let js = PaddleJsContext::new(&click, &mouse, &key, &pointer, &touch);
        let _closure_guards = vec![
            Box::new(click) as Box<dyn Any>,
            Box::new(mouse),
            Box::new(key),
            Box::new(pointer),
            Box::new(touch),
        ];

        let gate = EventGate {
            js,
            _closure_guards,
        };
        let aid = nuts::new_activity(gate);
        aid.private_channel(Self::register_event_listener);
    }
    pub fn listen<A: Activity>(frame: &FrameHandle<A>, event_type: EventListenerType) {
        nuts::send_to::<Self, _>(RegisterEventListener::new(frame, event_type));
    }
    fn register_event_listener(&mut self, msg: RegisterEventListener) {
        match msg.event_type {
            EventListenerType::Click => {
                let html = msg.div.parent_element().unwrap();
                let aid = msg.activity.as_usize();
                for event in ClickEventType::iter() {
                    self.js.register_click_event_listener(event, &html, aid);
                }
            }
            EventListenerType::Mouse => {
                let html = msg.div.parent_element().unwrap();
                let aid = msg.activity.as_usize();
                for event in MouseEventType::iter() {
                    self.js.register_mouse_event_listener(event, &html, aid);
                }
            }
            EventListenerType::Touch => {
                let html = msg.div.parent_element().unwrap();
                let aid = msg.activity.as_usize();
                for event in TouchEventType::iter() {
                    self.js.register_touch_event_listener(event, &html, aid);
                }
            }
            EventListenerType::BrowserPointer => {
                let html = msg.div.parent_element().unwrap();
                let aid = msg.activity.as_usize();
                for event in BrowserPointerEventType::iter() {
                    self.js.register_pointer_event_listener(event, &html, aid);
                }
            }
            EventListenerType::Keyboard => {
                let aid = msg.activity.as_usize();
                for event in KeyEventType::iter() {
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
