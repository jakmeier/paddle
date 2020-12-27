use super::keys::*;
use crate::Vector;
use strum_macros::EnumIter;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Event type and cursor position
pub struct PointerEvent(pub PointerEventType, pub Vector);
impl PointerEvent {
    pub fn event_type(&self) -> PointerEventType {
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

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, EnumIter)]
#[repr(u8)]
/// Rust representation for key event types.
/// Has a one-to-one correspondence to browser events.
pub enum KeyEventType {
    KeyDown,
    KeyPress,
    KeyUp,
}

/// Rust representation for mouse and touch event types.
///
/// Several different browser events are mapped to this unified pointer representation.
/// For example, all three of `mousemove`, `touchmove`, and `pointermove` are mapped to `PointerEventType::Move`.
/// If the browser generates multiple events mapped to the same `PointerEventType`, Paddle makes an effort to detect this and only forward one of them.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum PointerEventType {
    /// Left-click or short tap
    PrimaryClick,
    /// Right-click or long touch
    SecondaryClick,
    /// Double-click or double-tap
    DoubleClick,
    /// Release mouse or touch
    Up,
    /// Press mouse or touch
    Down,
    /// Pointer is moved to a new coordinate
    Move,
    /// Pointer is moved from outside the frame to inside the frame
    Enter,
    /// Pointer is moved from inside the frame to outside the frame
    Leave,
}
