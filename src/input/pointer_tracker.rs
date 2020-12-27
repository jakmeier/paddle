use crate::{PointerEvent, PointerEventType, Vector};

/// Helper struct that can be added to a frame to track advanced cursor updates.
///
/// This can be useful to make the cursor position available to the draw function, for example to draw hover effects.
/// Drag gestures are also supported, to perform e.g. scrolling.
///
/// **Usage**: Add it as a field to a frame and then call `track_pointer_event()` from the mouse method of the frame.
/// Then read e.g. position from the field in any of the frame methods.
#[derive(Copy, Clone, Debug, Default)]
pub struct PointerTracker {
    pos: Option<Vector>,
    mouse_down: Option<Vector>,
    drag: Drag,
}

impl PointerTracker {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pos(&self) -> Option<Vector> {
        self.pos
    }
    /// Movement from point A to point B with touch or pressed mouse.
    ///
    /// Reading a drag with this method clears the content.
    /// While moving the cursor, a partial drag are generated each frame.
    /// When multiple drags are generated before the user clear it, they are all summarized to one single movement.
    pub fn take_drag(&mut self) -> Option<(Vector, Vector)> {
        self.drag.take()
    }
    pub fn track_pointer_event(&mut self, event: &PointerEvent) {
        match event.event_type() {
            PointerEventType::Move => {
                let to = event.pos();
                self.pos = Some(to);
                if let Some(from) = self.mouse_down {
                    self.drag.add(from, to);
                    self.mouse_down = Some(to);
                }
            }
            PointerEventType::Leave => {
                self.pos = None;
            }
            PointerEventType::Down => {
                self.mouse_down = Some(event.pos());
            }
            PointerEventType::Up => {
                self.mouse_down = None;
            }
            _ => { /* NOP */ }
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]

struct Drag(Option<(Vector, Vector)>);

impl Drag {
    pub fn add(&mut self, start: Vector, end: Vector) {
        if let Some(old) = self.0 {
            self.0 = Some((old.0, end));
        } else {
            self.0 = Some((start, end));
        }
    }
    pub fn take(&mut self) -> Option<(Vector, Vector)> {
        self.0.take()
    }
}
