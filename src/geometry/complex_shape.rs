use crate::{AbstractMesh, Rectangle, Tessellate};

/// A single mesh of triangles ready to be drawn
pub struct ComplexShape {
    /// Normalized mesh defining the shape
    mesh: AbstractMesh,
    /// Natural size used for drawing without scaling
    bounding_box: Rectangle,
}

impl ComplexShape {
    pub fn new(mut mesh: AbstractMesh, bounding_box: Rectangle) -> Self {
        mesh.normalize(&bounding_box);
        Self { mesh, bounding_box }
    }
    pub fn resize(&mut self, bounding_box: &Rectangle) {
        self.bounding_box = *bounding_box;
    }
}

impl Tessellate for ComplexShape {
    fn tessellate<'a>(&self, mesh: &mut AbstractMesh) {
        mesh.extend(&self.mesh);
    }
    fn bounding_box(&self) -> Rectangle {
        self.bounding_box
    }
}
