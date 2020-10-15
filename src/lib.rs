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
pub fn utc_now() -> chrono::NaiveDateTime {
    let millis: f64 = js_sys::Date::now();
    let seconds = (millis / 1000.0).trunc() as i64;
    let nanos = ((millis % 1000.0) * 1_000_000.0) as u32;
    chrono::NaiveDateTime::from_timestamp(seconds, nanos)
}
