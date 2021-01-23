//! Triangles ready to be drawn by GPU, after tesselation and all CPU-side transformations have finished

use crate::graphics::AbstractTriangle;
use crate::Scalar;
use std::cmp::Ordering;

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
