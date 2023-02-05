//! Glue module between paddle.js and the Rust side of Paddle

use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use web_sys::HtmlElement;

use crate::input::browser_pointer_events::*;
use crate::*;

pub type ClickEventGateFnType = dyn FnMut(usize, ClickEventType, f32, f32);
pub type MouseEventGateFnType = dyn FnMut(usize, MouseEventType, f32, f32);
pub type KeyboardEventGateFnType = dyn FnMut(usize, KeyEventType, Key);
pub type PointerEventGateFnType = dyn FnMut(usize, BrowserPointerEventType, f32, f32);
pub type TouchEventGateFnType = dyn FnMut(usize, TouchEventType, f32, f32);

#[wasm_bindgen(module = "/src/js/paddle.js")]
extern "C" {
    pub type PaddleJsContext;

    #[wasm_bindgen(constructor)]
    pub fn new(
        click_event_gate: &Closure<ClickEventGateFnType>,
        mouse_event_gate: &Closure<MouseEventGateFnType>,
        keyboard_event_gate: &Closure<KeyboardEventGateFnType>,
        pointer_event_gate: &Closure<PointerEventGateFnType>,
        touch_event_gate: &Closure<TouchEventGateFnType>,
    ) -> PaddleJsContext;

    #[wasm_bindgen(method)]
    #[wasm_bindgen(js_name = registerClickEventListener)]
    pub fn register_click_event_listener(
        this: &PaddleJsContext,
        event_type: ClickEventType,
        listener: &HtmlElement,
        callback_id: usize,
    );
    #[wasm_bindgen(method)]
    #[wasm_bindgen(js_name = registerMouseEventListener)]
    pub fn register_mouse_event_listener(
        this: &PaddleJsContext,
        event_type: MouseEventType,
        listener: &HtmlElement,
        callback_id: usize,
    );
    #[wasm_bindgen(method)]
    #[wasm_bindgen(js_name = registerTouchEventListener)]
    pub fn register_touch_event_listener(
        this: &PaddleJsContext,
        event_type: TouchEventType,
        listener: &HtmlElement,
        callback_id: usize,
    );
    #[wasm_bindgen(method)]
    #[wasm_bindgen(js_name = registerPointerEventListener)]
    pub fn register_pointer_event_listener(
        this: &PaddleJsContext,
        event_type: BrowserPointerEventType,
        listener: &HtmlElement,
        callback_id: usize,
    );

    #[wasm_bindgen(method)]
    #[wasm_bindgen(js_name = registerKeyboardEventListener)]
    pub fn register_keyboard_event_listener(
        this: &PaddleJsContext,
        event_type: KeyEventType,
        callback_id: usize,
    );

    #[wasm_bindgen(js_name = supportsPointerEvents)]
    pub fn supports_pointer_events() -> bool;
}

#[wasm_bindgen(module = "/src/js/enums.js")]
extern "C" {
    fn mouseEventString();
}
