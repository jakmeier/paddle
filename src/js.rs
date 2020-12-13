//! Glue module between paddle.js and the Rust side of Paddle

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlElement;

use crate::{KeyEventType, MouseEventType};

#[wasm_bindgen(module = "/src/js/paddle.js")]
extern "C" {
    pub type PaddleJsContext;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PaddleJsContext;

    #[wasm_bindgen(method)]
    #[wasm_bindgen(js_name = registerMouseEventListener)]
    pub fn register_mouse_event_listener(
        this: &PaddleJsContext,
        event_type: MouseEventType,
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
}

#[wasm_bindgen(module = "/src/js/enums.js")]
extern "C" {
    fn mouseEventString();
}
