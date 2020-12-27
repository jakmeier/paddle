use crate::Vector;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Event type and cursor position
pub struct MouseEvent(pub MouseEventType, pub Vector);
impl MouseEvent {
    pub fn event_type(&self) -> MouseEventType {
        self.0
    }
    pub fn pos(&self) -> Vector {
        self.1
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct KeyEvent(pub KeyEventType, pub Key);
impl KeyEvent {
    pub fn event_type(&self) -> KeyEventType {
        self.0
    }
    pub fn key(&self) -> Key {
        self.1
    }
}

pub enum EventListenerType {
    KeyBoard(Vec<KeyEventType>),
    Mouse(Vec<MouseEventType>),
    // Possible extension:
    // BrowserEvent
}
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
/// Rust representation for key event types.
/// Has a one-to-one correspondence to browser events.
pub enum KeyEventType {
    KeyDown,
    KeyPress,
    KeyUp,
}
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
/// Rust representation for mouse event types.
/// Has a one-to-one correspondence to browser events.
pub enum MouseEventType {
    LeftClick,
    RightClick,
    DoubleClick,
    Up,
    Down,
    Move,
    /// Cursor is moved from outside the frame to inside the frame
    Enter,
    /// Cursor is moved from inside the frame to outside the frame
    Leave,
}

#[wasm_bindgen(js_name = KeyEnum)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
/// Rust representation of a set of common keys.
/// The names match the [Key Code Values](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code/code_values).
/// The keys listed should all have the same representation on all platforms.
///
/// For older browsers that don't support the `code` value, a conversion from the `key` value is done with best effort. This may not consider keyboard layouts perfectly.
pub enum Key {
    // These values are the same in event.key and event.code
    Escape,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    End,
    Home,
    PageDown,
    PageUp,
    Enter,
    Tab,
    Backspace,
    Delete,
    // These values need to be mapped from event.key to event.code for compatibility
    Space,
    AltLeft,
    AltRight,
    ShiftLeft,
    ShiftRight,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
}
