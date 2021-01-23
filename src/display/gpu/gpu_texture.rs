use web_sys::WebGlTexture;

use crate::graphics::Image;
use crate::{Rectangle, Vector};

#[derive(Clone, Debug)]
pub struct TexturePosition {
    /// normalized texture coordinate
    pub st: Vector,
    pub tex: WebGlTexture,
}

impl TexturePosition {
    pub fn new(st: Vector, tex: WebGlTexture) -> Self {
        Self { st, tex }
    }
}

impl Image {
    pub fn sample(&self, bounding_box: &Rectangle, position: &Vector) -> TexturePosition {
        let w = bounding_box.width();
        let h = bounding_box.height();
        let x = position.x - bounding_box.pos.x;
        let y = position.y - bounding_box.pos.y;
        let s = (x / w) * self.region.width() + self.region.x();
        let t = (y / h) * self.region.height() + self.region.y();
        TexturePosition::new((s, t).into(), self.texture.webgl_texture().clone())
    }
}
