use super::{primitives::GpuTriangle, GpuVertex};
use crate::{quicksilver_compat::Background, Transform, Vector, Z_MAX};

/// Buffer for GPU primitives ready to be drawn, after tesselation and all CPU-side transformations have finished
pub struct GpuMesh {
    pub vertices: Vec<GpuVertex>,
    pub triangles: Vec<GpuTriangle>,
}

impl GpuMesh {
    /// Create a new, empty mesh
    ///
    /// This allocates, so hold on to meshes rather than creating and destroying them
    pub fn new() -> GpuMesh {
        GpuMesh {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    /// Clear the mesh, removing anything that has been drawn to it
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.triangles.clear();
    }

    /// Add vertices from an iterator, some transforms, and a background
    pub fn add_positioned_vertices(
        &mut self,
        vertices: impl Iterator<Item = Vector>,
        z: f32,
        trans: Transform,
        tex_trans: Option<Transform>,
        bkg: Background,
    ) -> u32 {
        let offset = self.vertices.len();
        self.vertices.extend(
            vertices.map(|v| GpuVertex::new(trans * v, z, tex_trans.map(|trans| trans * v), bkg)),
        );
        offset as u32
    }

    /// Sets the z value for all vertices in the Mesh
    pub fn set_z(&mut self, z: i16) {
        for v in &mut self.vertices {
            v.z = z as f32 / Z_MAX as f32;
        }
        for t in &mut self.triangles {
            t.z = z as f32 / Z_MAX as f32;
        }
    }

    /// Scales all vertices in the mesh by the given factor, taking (0,0) as origin
    pub fn scale(&mut self, r: f32) {
        for p in self.vertices.iter_mut() {
            p.pos *= r;
            if let Some(mut tp) = p.tex_pos {
                tp *= r;
            }
        }
    }
}
