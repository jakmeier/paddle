//! This module has a copy of quicksilver types  to ease transition form quicksilver to something else.
//!
//! Everything inside has been copied from https://github.com/ryanisaacg/quicksilver and only marginally modified.
//!
pub mod geom;
pub mod graphics;

pub use super::input::*;
pub use geom::{about_equal, Circle, Shape};
pub use graphics::*;
