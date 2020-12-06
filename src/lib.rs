//! Paddle is a framework for easy game building for the browser.
//! TODO: more description, README
//!

pub use nuts;

#[macro_use]
pub(crate) mod debug;

pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod frame; // TODO: Probably rename (e.g. to activity)
pub mod graphics;
pub(crate) mod grid;
pub(crate) mod input;
pub(crate) mod jmr_geometry;
pub(crate) mod js;
pub(crate) mod load;
pub mod quicksilver_compat;
pub(crate) mod view_manager;
pub mod web_integration;

mod display;
pub use display::*;

pub use context::*;
pub use error::*;
pub use frame::*;
pub use input::*;
pub use jmr_geometry::*;
pub use load::*;
pub use view_manager::*;

// Code that currently belongs nowhere
pub fn utc_now() -> chrono::NaiveDateTime {
    let millis: f64 = js_sys::Date::now();
    let seconds = (millis / 1000.0).trunc() as i64;
    let nanos = ((millis % 1000.0) * 1_000_000.0) as u32;
    chrono::NaiveDateTime::from_timestamp(seconds, nanos)
}

pub fn init(config: PaddleConfig) -> PaddleResult<()> {
    web_integration::register_debug_hook();
    if config.enable_text_board {
        crate::TextBoard::init();
        enable_nuts_checks();
    }
    Context::init(config)?;
    EventGate::init();
    FrameManipulator::init();
    Ok(())
}
