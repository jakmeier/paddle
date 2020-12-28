use crate::quicksilver_compat::graphics::{Background::Col, Color};
use crate::Vector;
use lyon::tessellation::{
    geometry_builder::{Count, GeometryBuilder, GeometryBuilderError, VertexId},
    FillVertex, StrokeVertex, VertexConstructor,
};

use super::{AbstractMesh, AbstractTriangle, AbstractVertex};

/// A way to render complex shapes using the lyon API
///
/// The ShapeRenderer has a color which applies to all incoming shapes.
/// It outputs the shapes to a mutable AbstractMesh reference
pub struct ShapeRenderer<'a> {
    mesh: &'a mut AbstractMesh,
    color: Color,
    dirty: Option<usize>,
}

impl<'a> ShapeRenderer<'a> {
    /// Create a shape renderer with a target mesh and an initial color
    pub fn new(mesh: &'a mut AbstractMesh, color: Color) -> ShapeRenderer<'a> {
        ShapeRenderer {
            mesh,
            color,
            dirty: None,
        }
    }

    /// Get the current color of the incoming shapes
    pub fn color(&self) -> Color {
        self.color
    }

    /// Set the color of the incoming shapes
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl<'a, Input> GeometryBuilder<Input> for ShapeRenderer<'a>
where
    Color: VertexConstructor<Input, AbstractVertex>,
{
    fn begin_geometry(&mut self) {
        assert!(self.dirty.is_none());
        self.dirty = Some(self.mesh.triangles.len());
    }

    fn end_geometry(&mut self) -> Count {
        let dirty = self
            .dirty
            .expect("begin_geometry must be called before end_geometry");
        self.dirty = None;
        Count {
            vertices: self.mesh.vertices[dirty..].len() as u32,
            indices: self.mesh.triangles[dirty..].len() as u32 * 3,
        }
    }

    fn add_vertex(&mut self, vertex: Input) -> Result<VertexId, GeometryBuilderError> {
        let vertex = self.color.new_vertex(vertex);
        self.mesh.vertices.push(vertex);
        Ok(VertexId(self.mesh.vertices.len() as u32 - 1))
    }

    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        let triangle = AbstractTriangle::new(0, [a.0, b.0, c.0], Col(Color::WHITE));
        self.mesh.triangles.push(triangle);
    }

    fn abort_geometry(&mut self) {
        let dirty = self
            .dirty
            .expect("begin_geometry must be called before abort_geometry");
        self.dirty = None;
        self.mesh.vertices.truncate(dirty);
        self.mesh.triangles.truncate(dirty);
    }
}

impl VertexConstructor<FillVertex, AbstractVertex> for Color {
    fn new_vertex(&mut self, vertex: FillVertex) -> AbstractVertex {
        let position = Vector::new(vertex.position.x, vertex.position.y);
        AbstractVertex::new(position, None, Col(*self))
    }
}

impl VertexConstructor<StrokeVertex, AbstractVertex> for Color {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> AbstractVertex {
        let position = Vector::new(vertex.position.x, vertex.position.y);
        AbstractVertex::new(position, None, Col(*self))
    }
}
