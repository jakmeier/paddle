//! For storing tesselation results which can be drawn multiple times with different transformations

use crate::{Rectangle, Transform, Vector};

/// A mesh. If it is normalized, all values are within the bounding box: x,y in [-1,+1]
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

    /// Copy triangles from one mesh into another
    pub fn extend(&mut self, other: &Self) {
        let offset = self.vertices.len() as u32;
        self.vertices.extend(&other.vertices);
        self.triangles.extend(other.triangles.iter().map(|t| {
            let mut t = t.clone();
            t.add_offset(offset);
            t
        }));
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
        vertices: impl IntoIterator<Item = Vector>,
        trans: Transform,
    ) -> u32 {
        let offset = self.vertices.len();
        self.vertices
            .extend(vertices.into_iter().map(|v| AbstractVertex::new(trans * v)));
        offset as u32
    }

    /// Scales all vertices in the mesh by the given factor, taking (0,0) as origin
    pub fn scale(&mut self, r: f32) {
        for p in self.vertices.iter_mut() {
            p.pos *= r;
        }
    }

    /// Transforms the mesh from a coordinate space within `bounding_box` into the normalized space (x,y in [-1,+1])
    pub fn normalize(&mut self, bounding_box: &Rectangle) {
        let min_x = bounding_box.pos.x;
        let min_y = bounding_box.pos.y;
        let max_x = bounding_box.pos.x + bounding_box.size.x;
        let max_y = bounding_box.pos.y + bounding_box.size.y;

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
    pub fn add_offset(&mut self, offset: u32) {
        self.indices[0] += offset;
        self.indices[1] += offset;
        self.indices[2] += offset;
    }
}
