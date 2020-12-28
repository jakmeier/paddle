//! GPU primitives ready to be drawn, after tesselation and all CPU-side transformations have finished

use crate::graphics::{AbstractTriangle, Image};
use crate::quicksilver_compat::graphics::{Background, Color};
use crate::{Scalar, Vector};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
/// A vertex for drawing items to the GPU
pub struct GpuVertex {
    /// The position of the vertex in space
    pub pos: Vector,
    /// If there is a texture attached to this vertex, where to get the texture data from
    ///
    /// It is normalized from 0 to 1
    pub tex_pos: Option<Vector>,
    /// The color to blend this vertex with
    pub col: Color,
    /// Z coordinate in range [-1,1]
    pub z: f32,
}

impl GpuVertex {
    /// Create a new GPU vertex
    pub fn new(
        pos: impl Into<Vector>,
        z: f32,
        tex_pos: Option<Vector>,
        bkg: Background,
    ) -> GpuVertex {
        GpuVertex {
            pos: pos.into(),
            tex_pos,
            col: bkg.color(),
            z,
        }
    }
}

#[derive(Clone)]
/// A triangle to draw to the GPU
pub struct GpuTriangle {
    /// The plane the triangle falls on
    pub z: f32,
    /// The indexes in the vertex list that the GpuTriangle uses
    pub indices: [u32; 3],
    /// The (optional) image used by the GpuTriangle
    ///
    /// All of the vertices used by the triangle should agree on whether it uses an image,
    /// it is up to you to maintain this
    pub image: Option<Image>,
}

impl GpuTriangle {
    /// Create a new untextured GPU Triangle
    pub fn new(offset: u32, indices: [u32; 3], z: impl Scalar, bkg: Background) -> GpuTriangle {
        GpuTriangle {
            z: z.float(),
            indices: [
                indices[0] + offset,
                indices[1] + offset,
                indices[2] + offset,
            ],
            image: bkg.image().cloned(),
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
            image: t.image.clone(),
        }
    }
}

impl PartialEq for GpuTriangle {
    fn eq(&self, other: &GpuTriangle) -> bool {
        match (&self.image, &other.image) {
            (&Some(ref a), &Some(ref b)) => a == b,
            (&None, &None) => true,
            _ => false,
        }
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
        match self.z.partial_cmp(&other.z) {
            None | Some(Ordering::Equal) => match (&self.image, &other.image) {
                (&Some(_), &Some(_)) => Ordering::Equal,
                (&Some(_), &None) => Ordering::Greater,
                (&None, &Some(_)) => Ordering::Less,
                (&None, &None) => Ordering::Equal,
            },
            Some(result) => result,
        }
    }
}
