pub enum EventType {
    Key(KeyEventType),
    Mouse(MouseEventType),
    // Possible extension:
    // BrowserEvent
}
pub enum KeyEventType {
    KeyDown,
    // KeyPress,
    KeyUp,
}
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum MouseEventType {
    LeftClick = 1,
    RightClick = 2,
    DoubleClick = 3,
    Down = 4,
    // Enter = 5,
    // Leave = 6,
    Move = 7,
    // Over = 8,
    // Out = 9,
    Up = 10,
}

// TODO: These need to be created in JS from a string value
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Key {
    Alt,
    AltGraph,
    Shift,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Digit0,
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
    Space,
}
