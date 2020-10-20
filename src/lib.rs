//! Paddle is a framework for easy game building for the browser.
//! TODO: more description, README
//!

pub use nuts;

pub(crate) mod canvas;
pub(crate) mod error;
pub(crate) mod event;
pub(crate) mod frame; // TODO: Probably rename (e.g. to activity)
pub(crate) mod grid;
pub(crate) mod jmr_geometry;
pub(crate) mod load;
pub mod quicksilver_compat;
pub(crate) mod text;
pub(crate) mod view_manager;

pub use canvas::*;
pub use error::*;
pub use event::*;
pub use frame::*;
pub use jmr_geometry::*;
pub use load::*;
pub use text::*;
pub use view_manager::*;

// Code that currently belongs nowhere

use stdweb::js;
use stdweb::unstable::TryInto;
pub fn utc_now() -> chrono::NaiveDateTime {
    let millis: f64 = js!(
        var date = new Date();
        return date.getTime();
    )
    .try_into()
    .expect("Reading time");
    let seconds = (millis / 1000.0).trunc() as i64;
    let nanos = ((millis % 1000.0) * 1_000_000.0) as u32;
    chrono::NaiveDateTime::from_timestamp(seconds, nanos)
}

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
