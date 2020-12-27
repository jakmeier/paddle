//! Rust representations of pointing device related events, as defined by w3c standards
//!
//! Touch, mouse, and pointer events are all handled to ensure maximal compatibility across browsers and devices.

use super::event::PointerEventType;
use strum_macros::EnumIter;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, EnumIter)]
#[repr(u8)]
/// Rust representation for click event types.
/// Has a one-to-one correspondence to browser events.
pub enum ClickEventType {
    LeftClick,
    RightClick,
    DoubleClick,
}
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, EnumIter)]
#[repr(u8)]
/// Rust representation for mouse event types.
/// Has a one-to-one correspondence to browser events.
pub enum MouseEventType {
    Up,
    Down,
    Move,
    Enter,
    Leave,
}
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, EnumIter)]
#[repr(u8)]
/// Rust representation for touch event types.
/// Has a one-to-one correspondence to browser events.
pub enum TouchEventType {
    Start,
    End,
    Move,
    Cancel,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, EnumIter)]
#[repr(u8)]
/// Rust representation for pointer event types.
/// Has a one-to-one correspondence to browser events.
pub enum BrowserPointerEventType {
    Down,
    Up,
    Move,
    Enter,
    Leave,
    Cancel,
}

impl Into<PointerEventType> for ClickEventType {
    fn into(self) -> PointerEventType {
        match self {
            ClickEventType::LeftClick => PointerEventType::PrimaryClick,
            ClickEventType::RightClick => PointerEventType::SecondaryClick,
            ClickEventType::DoubleClick => PointerEventType::DoubleClick,
        }
    }
}
impl Into<PointerEventType> for MouseEventType {
    fn into(self) -> PointerEventType {
        match self {
            MouseEventType::Up => PointerEventType::Up,
            MouseEventType::Down => PointerEventType::Down,
            MouseEventType::Move => PointerEventType::Move,
            MouseEventType::Enter => PointerEventType::Enter,
            MouseEventType::Leave => PointerEventType::Leave,
        }
    }
}
impl Into<PointerEventType> for TouchEventType {
    fn into(self) -> PointerEventType {
        match self {
            TouchEventType::Start => PointerEventType::Down,
            TouchEventType::End => PointerEventType::Up,
            TouchEventType::Move => PointerEventType::Move,
            TouchEventType::Cancel => PointerEventType::Leave,
        }
    }
}
impl Into<PointerEventType> for BrowserPointerEventType {
    fn into(self) -> PointerEventType {
        match self {
            BrowserPointerEventType::Down => PointerEventType::Down,
            BrowserPointerEventType::Up => PointerEventType::Up,
            BrowserPointerEventType::Move => PointerEventType::Move,
            BrowserPointerEventType::Enter => PointerEventType::Enter,
            BrowserPointerEventType::Leave => PointerEventType::Leave,
            BrowserPointerEventType::Cancel => PointerEventType::Leave,
        }
    }
}
