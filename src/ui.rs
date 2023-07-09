//! User Interface: Everything related to placing and manipulating elements that
//! the user can interact with.
//!
//! For now, the UI elements available are fairly basic. But eventually, it is
//! supposed to include a large range of typical UI elements such as buttons,
//! texts, tables, and so on. But also, it shall cover rendered elements such as
//! animated characters.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::quicksilver_compat::Shape;
use crate::{
    ComplexShape, DisplayPaint, FitStrategy, FloatingText, PaddleResult, PointerEventType,
    Rectangle, Transform,
};

/// A logical element for display and interaction
pub struct UiElement {
    /// The hit box and usually also constraining area of the element.
    /// Relative to the parent frame.
    area: Rectangle,
    /// Drawing area, defined by a mesh of triangles.
    shape: ComplexShape,
    /// How to color the triangles.
    paint: Box<dyn DisplayPaint>,
    /// Zero or one text to display on the element.
    text: Option<RefCell<FloatingText>>,
    /// Registered interactive events and what to do on them.
    triggers: HashMap<PointerEventType, Box<dyn Fn()>>,
}

impl UiElement {
    pub fn new(area: Rectangle, paint: impl DisplayPaint + 'static) -> Self {
        Self {
            area,
            paint: Box::new(paint),
            shape: ComplexShape::from_shape(area),
            triggers: Default::default(),
            text: None,
        }
    }

    pub fn with_text(mut self, text: String) -> PaddleResult<Self> {
        let float = FloatingText::new(&self.area, text)?;
        self.text = Some(RefCell::new(float));
        Ok(self)
    }

    pub fn with_text_alignment(self, fit: FitStrategy) -> PaddleResult<Self> {
        let text = self
            .text
            .as_ref()
            .ok_or_else(|| crate::ErrorMessage::technical("No text to be aligned".to_owned()))?;
        text.borrow_mut().update_fit_strategy(fit)?;
        Ok(self)
    }

    /// Publish the message object every time the UI element observes the
    /// pointer interaction.
    ///
    /// Only one message per interaction is possible. This function will
    /// currently panic if a second message is registered on the same event
    /// type. This behaviour might change in the future.
    pub fn with_pointer_interaction<T: 'static + Clone>(
        mut self,
        trigger: PointerEventType,
        msg: T,
    ) -> Self {
        let prev = self
            .triggers
            .insert(trigger, Box::new(move || crate::share(msg.clone())));
        // Panicking here makes changing the behaviour in the future easier, as
        // we can decide to overwrite or append without breaking old correct
        // code.
        assert!(prev.is_none(), "event already has a message to publish");
        self
    }

    pub fn with_rounded_corners(mut self, radius: f32) -> Self {
        self.shape = ComplexShape::rounded_rectangle(self.area, radius);
        self
    }

    pub fn draw(&self, canvas: &mut crate::DisplayArea) {
        let z = 0;
        canvas.draw_ex(&self.shape, &self.paint, Transform::IDENTITY, z);
        if let Some(text) = self.text.as_ref() {
            text.borrow_mut()
                .update_position(&canvas.frame_to_display_area(self.area), 0)
                .unwrap();
            text.borrow_mut().draw();
        }
    }

    pub fn pointer(&self, evt: crate::PointerEvent) {
        if self.triggers.is_empty() {
            return;
        }
        if evt.pos().overlaps(&self.area) {
            if let Some(trigger) = self.triggers.get(&evt.event_type()) {
                trigger()
            }
        }
    }
}