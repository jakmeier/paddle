//! Triangles ready to be drawn by GPU, after tesselation and all CPU-side transformations have finished

use crate::graphics::AbstractTriangle;
use crate::{Scalar, UniformList};
use std::cmp::Ordering;

#[derive(Clone)]
/// A triangle to draw to the GPU
pub struct GpuTriangle {
    /// The plane the triangle falls on
    pub z: f32,
    /// The indexes in the vertex list that the GpuTriangle uses
    pub indices: [u32; 3],
    /// Uniforms the triangles needs to be set.
    pub uniforms: UniformList,
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
            uniforms: UniformList::default(),
        }
    }
    pub fn from_abstract(t: &AbstractTriangle, offset: u32, z: f32, uniforms: UniformList) -> Self {
        Self {
            z,
            uniforms,
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
