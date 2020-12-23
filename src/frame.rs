//! Frames are active areas, like light-weight windows.
//!
//! Frames are the number one interface of the paddle game engine to developers using it.
//! Games built with Paddlers should be structured around them.
//!
//! Game updates are scheduled through frames.
//! Draw calls are typically done inside frames.
//! User input is received through frames.
//!
//! Each frame has a dynamic position on the display.
//! Drawing and user input is restricted to that area.
//!
//! Frames can also be put in the background, in which state reduced events are handled and nothing is drawn.

use crate::quicksilver_compat::{Rectangle, Vector};
use crate::{DisplayArea, KeyEvent};
use nuts::*;

mod frame_manipulation;
pub(crate) use frame_manipulation::*;
mod frame_registration;
pub use frame_registration::*;
mod scheduling;
pub use scheduling::*;

/// A frame takes up some area on the screen where it is drawn and reacts to UI events.
/// The position and size of a frame is static for now. (Static in game coordinates, it will adapt properly to screen resizing etc.)
pub trait Frame {
    type State;
    const WIDTH: u32;
    const HEIGHT: u32;
    fn draw(&mut self, _state: &mut Self::State, _canvas: &mut DisplayArea, _timestamp: f64) {}
    fn update(&mut self, _state: &mut Self::State) {}
    fn leave(&mut self, _state: &mut Self::State) {}
    fn enter(&mut self, _state: &mut Self::State) {}
    fn left_click(&mut self, _state: &mut Self::State, _pos: (i32, i32)) {}
    fn right_click(&mut self, _state: &mut Self::State, _pos: (i32, i32)) {}
    fn mouse_move(&mut self, _state: &mut Self::State, _pos: (i32, i32)) {}
    fn key(&mut self, _state: &mut Self::State, _key: KeyEvent) {}

    #[inline(always)]
    fn size() -> Vector {
        Vector::new(Self::WIDTH, Self::HEIGHT)
    }
    #[inline(always)]
    fn area() -> Rectangle {
        Rectangle::new_sized(Self::size())
    }
}

/// Handle to frame is returned when adding it to the view manager.
/// Can be used to add listeners. (And potentially other manipulations of the frame are possible in the future)
#[derive(Clone)]
pub struct FrameHandle<FRAME> {
    activity_id: ActivityId<FRAME>,
    div: div::PaneHandle,
    region: Rectangle,
}

impl<FRAME> FrameHandle<FRAME> {
    pub fn new(activity_id: ActivityId<FRAME>, div: div::PaneHandle, region: Rectangle) -> Self {
        let fh = Self {
            activity_id,
            div,
            region,
        };
        #[cfg(debug_assertions)]
        fh.set_id(std::any::type_name::<FRAME>());
        fh
    }
    pub fn activity(&self) -> ActivityId<FRAME> {
        self.activity_id
    }
    pub fn div(&self) -> &div::PaneHandle {
        &self.div
    }
    pub fn region(&self) -> Rectangle {
        self.region
    }
    #[cfg(debug_assertions)]
    fn set_id(&self, id: &str) {
        let parent = self.div.parent_element().unwrap();
        parent.set_id(id);
    }
}

#[derive(Clone, Copy)]
pub enum Domain {
    Frame,
    Network,
}
domain_enum!(Domain);

/// Goes to active and inactive frames
struct GlobalEvent<Ev>(pub(crate) Ev);
/// Goes to active frames only
struct ActiveEvent<Ev>(pub(crate) Ev);

/// Share anything with all other activities in background and foreground
pub fn share<MSG: 'static>(msg: MSG) {
    nuts::publish(GlobalEvent(msg));
}

/// Share anything with all foreground activities
pub fn share_foreground<MSG: 'static>(msg: MSG) {
    nuts::publish(ActiveEvent(msg));
}
