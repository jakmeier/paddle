//! Parent module for stuff related to graphics manipulation, such as textures and tessellation.
//! (exclusive display + GPU)
mod paint;
mod tessellation;
mod texture;

pub use paint::*;
pub use tessellation::*;
pub use texture::TextureConfig;
pub use texture::*;
