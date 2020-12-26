use super::Background;
use super::{GpuTriangle, Vertex};
use crate::{Transform, Vector, Z_MAX};

/// A way to store rendered objects without having to re-process them
pub struct Mesh {
    /// All the vertices in the mesh
    pub vertices: Vec<Vertex>,
    /// All the triangles in the mesh
    pub triangles: Vec<GpuTriangle>,
}

impl Mesh {
    /// Create a new, empty mesh
    ///
    /// This allocates, so hold on to meshes rather than creating and destroying them
    pub fn new() -> Mesh {
        Mesh {
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
            vertices.map(|v| Vertex::new(trans * v, z, tex_trans.map(|trans| trans * v), bkg)),
        );
        offset as u32
    }

    /// Add all the data from the other mesh into this mesh
    // pub fn extend(&mut self, other: &Mesh) {
    //     self.vertices.extend(other.vertices.iter().cloned());
    //     self.triangles.extend(other.triangles.iter().cloned());
    // }

    /// Sets the z value for all vertices in the Mesh
    pub fn set_z(&mut self, z: i16) {
        for v in &mut self.vertices {
            v.z = z as f32 / Z_MAX as f32;
        }
    }
}
