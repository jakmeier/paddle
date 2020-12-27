//! Glue module between paddle.js and the Rust side of Paddle

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;

use crate::input::browser_pointer_events::*;
use crate::*;

#[wasm_bindgen(module = "/src/js/paddle.js")]
extern "C" {
    pub type PaddleJsContext;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PaddleJsContext;

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
