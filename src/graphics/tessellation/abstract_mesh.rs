//! For storing tesselation results which can be drawn multiple times with different transformations

use crate::graphics::Image;
use crate::quicksilver_compat::graphics::{Background, Color};
use crate::{Transform, Vector};

/// A way to store rendered objects without having to re-process them
pub struct AbstractMesh {
    pub vertices: Vec<AbstractVertex>,
    pub triangles: Vec<AbstractTriangle>,
}

impl AbstractMesh {
    /// Create a new, empty mesh
    ///
    /// This allocates, so hold on to meshes rather than creating and destroying them
    pub fn new() -> AbstractMesh {
        AbstractMesh {
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
        trans: Transform,
        tex_trans: Option<Transform>,
        bkg: Background,
    ) -> u32 {
        let offset = self.vertices.len();
        self.vertices.extend(
            vertices.map(|v| AbstractVertex::new(trans * v, tex_trans.map(|trans| trans * v), bkg)),
        );
        offset as u32
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

#[derive(Clone, Copy, Debug)]
/// A vertex for drawing items to the GPU
pub struct AbstractVertex {
    /// The position of the vertex in space
    pub pos: Vector,
    /// If there is a texture attached to this vertex, where to get the texture data from
    ///
    /// It is normalized from 0 to 1
    pub tex_pos: Option<Vector>,
    /// The color to blend this vertex with
    pub col: Color,
}

impl AbstractVertex {
    /// Create a new abstract vertex
    pub fn new(pos: impl Into<Vector>, tex_pos: Option<Vector>, bkg: Background) -> AbstractVertex {
        AbstractVertex {
            pos: pos.into(),
            tex_pos,
            col: bkg.color(),
        }
    }
}

#[derive(Clone)]
/// Triangle in AbstractMesh
pub struct AbstractTriangle {
    /// The indexes in the vertex list that the AbstractTriangle uses
    pub indices: [u32; 3],
    /// The (optional) image used by the AbstractTriangle
    ///
    /// All of the vertices used by the triangle should agree on whether it uses an image,
    /// it is up to you to maintain this
    pub image: Option<Image>,
}

impl AbstractTriangle {
    /// Create a new untextured GPU Triangle
    pub fn new(offset: u32, indices: [u32; 3], bkg: Background) -> AbstractTriangle {
        AbstractTriangle {
            indices: [
                indices[0] + offset,
                indices[1] + offset,
                indices[2] + offset,
            ],
            image: bkg.image().cloned(),
        }
    }
}

impl PartialEq for AbstractTriangle {
    fn eq(&self, other: &AbstractTriangle) -> bool {
        match (&self.image, &other.image) {
            (&Some(ref a), &Some(ref b)) => a == b,
            (&None, &None) => true,
            _ => false,
        }
    }
}

impl Eq for AbstractTriangle {}
