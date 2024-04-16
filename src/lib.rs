//! Paddle is a framework for easy game building for the browser.
//! TODO: more description, README
//!
#![cfg_attr(feature = "nightly", feature(const_fn_floating_point_arithmetic))]

pub use nuts;

#[macro_use]
pub(crate) mod debug;

pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod frame;
pub mod graphics;
#[cfg(feature = "html_helpers")]
pub mod html;
pub(crate) mod input;
pub(crate) mod js;
pub(crate) mod load;
pub mod quicksilver_compat;
pub mod ui;
pub(crate) mod view_manager;
pub mod web_integration;

mod display;
mod geometry;
pub use context::*;
pub use display::*;
pub use error::*;
pub use frame::*;
pub use geometry::*;
pub use graphics::*;
pub use input::*;
pub use load::*;
pub use ui::*;
pub use view_manager::*;

// Code that currently belongs nowhere
pub fn utc_now() -> chrono::NaiveDateTime {
    let millis: f64 = js_sys::Date::now();
    chrono::DateTime::from_timestamp_millis(millis as i64)
        .unwrap()
        .naive_utc()
}

pub fn init(config: PaddleConfig) -> PaddleResult<()> {
    web_integration::register_debug_hook();
    if let Some(region) = config.text_board_region {
        crate::TextBoard::init(region);
        enable_nuts_checks_to_textboard();
    } else {
        enable_nuts_checks_to_console();
    }
    Context::init(config)?;
    EventGate::init();
    FrameManipulator::init();
    LoadActivity::init();
    Ok(())
}
