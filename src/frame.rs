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

use crate::*;
use nuts::*;

mod frame_manipulation;
pub(crate) use frame_manipulation::*;
mod frame_registration;
pub use frame_registration::*;
mod scheduling;
pub use scheduling::*;

/// A frame takes up some area on the screen where it is drawn and reacts to UI events.
///
/// Define trait methods to accept user input, react to frame lifecycle changes, and draw to the screen.
//
/// The position and size of a frame is static (for now). (Static in game coordinates, the actual size and position will adapt properly to screen resizing etc.)
pub trait Frame {
    type State;
    const WIDTH: u32;
    const HEIGHT: u32;
    fn draw(&mut self, _state: &mut Self::State, _canvas: &mut DisplayArea, _timestamp: f64) {}
    fn update(&mut self, _state: &mut Self::State) {}
    fn leave(&mut self, _state: &mut Self::State) {}
    fn enter(&mut self, _state: &mut Self::State) {}
    fn key(&mut self, _state: &mut Self::State, _key: KeyEvent) {}
    fn pointer(&mut self, _state: &mut Self::State, _event: PointerEvent) {}

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
    div: div::DivHandle,
    region: Rectangle,
}

impl<FRAME> FrameHandle<FRAME> {
    pub fn new(activity_id: ActivityId<FRAME>, div: div::DivHandle, region: Rectangle) -> Self {
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
    pub fn div(&self) -> &div::DivHandle {
        &self.div
    }
    pub fn region(&self) -> Rectangle {
        self.region
    }
    /// Define z-index property of HTML.
    ///
    /// This might be necessary to ensure user input is processed by the intended frame. Drawing order for WebGL is not affected.
    pub fn set_z(&self, z: i32) {
        self.div.set_css("z-index", &z.to_string()).nuts_check();
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
