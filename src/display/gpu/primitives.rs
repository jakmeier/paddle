//! GPU primitives ready to be drawn, after tesselation and all CPU-side transformations have finished

use web_sys::WebGlTexture;

use crate::graphics::{AbstractTriangle, Image};
use crate::{quicksilver_compat::graphics::Color, Rectangle};
use crate::{Scalar, Vector};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
/// A vertex for drawing items to the GPU
pub struct GpuVertex {
    /// The position of the vertex in space
    pub pos: Vector,
    /// The image to sample from when drawing the triangel.
    /// When no image is defined, just the colors on the vertices will be used.
    /// If both are defined, the image is blended on top of the colors.
    pub image: Option<TexturePoisition>,
    /// The color to blend this vertex with
    pub col: Color,
    /// Z coordinate in range [-1,1]
    pub z: f32,
}

#[derive(Clone, Debug)]
pub struct TexturePoisition {
    /// normalized texture coordinate
    pub st: Vector,
    pub tex: WebGlTexture,
}

impl TexturePoisition {
    pub fn new(st: Vector, tex: WebGlTexture) -> Self {
        Self { st, tex }
    }
}

impl Image {
    pub fn sample(&self, bounding_box: &Rectangle, position: &Vector) -> TexturePoisition {
        let w = bounding_box.width();
        let h = bounding_box.height();
        let x = position.x - bounding_box.pos.x;
        let y = position.y - bounding_box.pos.y;
        let s = (x / w + self.region.x()) / self.region.width();
        let t = (y / h + self.region.y()) / self.region.height();
        TexturePoisition::new((s, t).into(), self.texture.webgl_texture().clone())
    }
}

impl GpuVertex {
    pub fn new(pos: Vector, image: Option<TexturePoisition>, col: Color, z: f32) -> Self {
        Self { pos, image, col, z }
    }
    pub fn has_texture(&self) -> bool {
        self.image.is_some()
    }
    // ST-coordinates (normalized)
    pub fn tex_coordinate(&self) -> Option<Vector> {
        self.image.as_ref().map(|img| img.st)
    }
    pub fn tex(&self) -> Option<&WebGlTexture> {
        self.image.as_ref().map(|img| &img.tex)
    }
}

#[derive(Clone)]
/// A triangle to draw to the GPU
pub struct GpuTriangle {
    /// The plane the triangle falls on
    pub z: f32,
    /// The indexes in the vertex list that the GpuTriangle uses
    pub indices: [u32; 3],
}

impl GpuTriangle {
    /// Create a new untextured GPU Triangle
    pub fn new(offset: u32, indices: [u32; 3], z: impl Scalar) -> GpuTriangle {
        GpuTriangle {
            z: z.float(),
            indices: [
                indices[0] + offset,
                indices[1] + offset,
                indices[2] + offset,
            ],
        }
    }
    pub fn from_abstract(t: &AbstractTriangle, offset: u32, z: f32) -> Self {
        Self {
            z,
            indices: [
                t.indices[0] + offset,
                t.indices[1] + offset,
                t.indices[2] + offset,
            ],
        }
    }
}

// For sorting by z-order
impl PartialEq for GpuTriangle {
    fn eq(&self, other: &GpuTriangle) -> bool {
        self.z.eq(&other.z)
    }
}

impl Eq for GpuTriangle {}

impl PartialOrd for GpuTriangle {
    fn partial_cmp(&self, other: &GpuTriangle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GpuTriangle {
    fn cmp(&self, other: &GpuTriangle) -> Ordering {
        self.z.partial_cmp(&other.z).unwrap()
    }
}
