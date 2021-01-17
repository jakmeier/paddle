//! For storing tesselation results which can be drawn multiple times with different transformations

use crate::{Transform, Vector};

/// A mesh within the bounding box: x,y in [-1,+1]
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

    /// Clear the mesh and recycle it
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.triangles.clear();
    }

    /// Add traingles + vertices from an iterator
    pub fn add_triangles(&mut self, vertices: &[Vector], triangles: &[[u32; 3]]) -> u32 {
        let offset = self.vertices.len() as u32;
        self.vertices
            .extend(vertices.iter().cloned().map(AbstractVertex::new));
        self.triangles.extend(
            triangles
                .iter()
                .map(|indices| AbstractTriangle::new(offset, *indices)),
        );
        offset as u32
    }
    /// Add vertices from an iterator annd apply a transform to each vertex
    pub fn add_positioned_vertices(
        &mut self,
        vertices: impl Iterator<Item = Vector>,
        trans: Transform,
    ) -> u32 {
        let offset = self.vertices.len();
        self.vertices
            .extend(vertices.map(|v| AbstractVertex::new(trans * v)));
        offset as u32
    }

    /// Scales all vertices in the mesh by the given factor, taking (0,0) as origin
    pub fn scale(&mut self, r: f32) {
        for p in self.vertices.iter_mut() {
            p.pos *= r;
        }
    }

    /// Temporary solution to fix vertices that have been created in a different coordinate space than should be used for AbstractMesh
    pub fn normalize(&mut self) {
        let mut min_x = 0.0;
        let mut min_y = 0.0;
        let mut max_x = 0.0;
        let mut max_y = 0.0;
        for v in &self.vertices {
            min_x = v.pos.x.min(min_x);
            min_y = v.pos.y.min(min_y);
            max_x = v.pos.x.max(max_x);
            max_y = v.pos.y.max(max_y);
        }
        debug_assert_ne!(min_x, max_x, "Cannot normalize mesh with 0 area");
        debug_assert_ne!(min_y, max_y, "Cannot normalize mesh with 0 area");
        let offset = Vector::new(-min_x, -min_y);
        let scale = Vector::new(2.0 / (max_x - min_x), 2.0 / (max_y - min_y));
        let const_offset = Vector::new(-1.0, -1.0);
        for v in &mut self.vertices {
            v.pos += offset;
            v.pos = v.pos.times(scale);
            v.pos += const_offset;
        }
    }
}

#[derive(Clone, Copy, Debug)]
/// A vertex for a geometric shape
pub struct AbstractVertex {
    /// The position of the vertex in space. Boundaries defined by user.
    pub pos: Vector,
}

impl AbstractVertex {
    /// Create a new abstract vertex
    pub fn new(pos: impl Into<Vector>) -> AbstractVertex {
        AbstractVertex { pos: pos.into() }
    }
}

#[derive(Clone)]
/// Triangle in AbstractMesh
pub struct AbstractTriangle {
    /// The indexes in the vertex list that the AbstractTriangle uses
    pub indices: [u32; 3],
}

impl AbstractTriangle {
    pub fn new(offset: u32, indices: [u32; 3]) -> AbstractTriangle {
        AbstractTriangle {
            indices: [
                indices[0] + offset,
                indices[1] + offset,
                indices[2] + offset,
            ],
        }
    }
}
