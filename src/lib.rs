//! Paddle is a framework for easy game building for the browser.
//! TODO: more description, README
//!

pub use nuts;

pub(crate) mod frame; // TODO: Probably rename (e.g. to activity)
pub(crate) mod view_manager;

pub use frame::*;
pub use view_manager::*;

// Code that might be useful later but otherwise can be deleted

// /// Calls nuts::draw() in every animation frame as managed by the browser. (Using requestAnimationFrame)
// pub fn auto_draw() {
//     stdweb::web::window().request_animation_frame(|_| crate::draw());
// }

// /// Calls nuts::update() in intervals managed by the browser. (Using setInterval)
// /// The defined interval will be the maximum number of calls but may be less if the computation takes too long
// pub fn auto_update(delay_ms: u32) {
//     let callback = crate::update;

//     js!( @(no_return)
//         setInterval( @{callback}, @{delay_ms});
//     );
// }
