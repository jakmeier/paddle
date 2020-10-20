//! Currently not much more than quicksilver compat
use crate::quicksilver_compat::Vector;

/// The current state of a button
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ButtonState {
    /// The button was activated this frame
    Pressed = 0,
    /// The button is active but was not activated this frame
    Held = 1,
    /// The button was released this frame
    Released = 2,
    /// The button is not active but was not released this frame
    NotPressed = 3,
}

#[derive(Clone, Debug)]
/// An input event
pub enum Event {
    /// A key has changed its button state
    Key(Key, ButtonState),
    /// An alphanumeric character has been entered from the keyboard
    MouseMoved(Vector),
    /// A mouse button has changed its button state
    MouseButton(MouseButton, ButtonState),
}

#[derive(Clone, Debug, Copy)]
/// The different buttons a user can press on a mouse
pub enum MouseButton {
    /// The left mouse button
    Left = 0,
    /// The right mouse button
    Right = 1,
    /// The middle mouse button
    Middle = 2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// A simple mouse cursor abstraction
///
/// Mice are owned and maintained a `Window` and can be accessed via the `mouse` function.
pub struct Mouse {
    pub(crate) pos: Vector,
    pub(crate) buttons: [ButtonState; 3],
    pub(crate) wheel: Vector,
}

impl Mouse {
    pub fn init() -> Self {
        Self {
            pos: Default::default(),
            buttons: [
                ButtonState::NotPressed,
                ButtonState::NotPressed,
                ButtonState::NotPressed,
            ],
            wheel: Default::default(),
        }
    }
    ///The location of the cursor in the viewport space
    pub fn pos(&self) -> Vector {
        self.pos
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Key {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
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
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
}
