use crate::Vector;
use lyon::{
    lyon_tessellation::{
        FillGeometryBuilder, FillVertexConstructor, GeometryBuilderError, StrokeGeometryBuilder,
        StrokeVertexConstructor, VertexId,
    },
    tessellation::{
        geometry_builder::{Count, GeometryBuilder},
        FillVertex, StrokeVertex,
    },
};

use super::{AbstractMesh, AbstractTriangle, AbstractVertex};

/// A way to render complex shapes using the lyon API
pub struct ShapeRenderer<'a> {
    mesh: &'a mut AbstractMesh,
    dirty: Option<usize>,
}

impl<'a> ShapeRenderer<'a> {
    /// Create a shape renderer with a target mesh
    pub fn new(mesh: &'a mut AbstractMesh) -> ShapeRenderer<'a> {
        ShapeRenderer { mesh, dirty: None }
    }
}

impl<'a> GeometryBuilder for ShapeRenderer<'a> {
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

    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        let triangle = AbstractTriangle::new(0, [a.0, b.0, c.0]);
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

impl<'a> FillGeometryBuilder for ShapeRenderer<'a> {
    fn add_fill_vertex(&mut self, vertex: FillVertex) -> Result<VertexId, GeometryBuilderError> {
        let vertex = FillVertexConstructor::new_vertex(&mut (), vertex);
        self.mesh.vertices.push(vertex);
        Ok(VertexId(self.mesh.vertices.len() as u32 - 1))
    }
}

impl<'a> StrokeGeometryBuilder for ShapeRenderer<'a> {
    fn add_stroke_vertex(
        &mut self,
        vertex: StrokeVertex,
    ) -> Result<VertexId, GeometryBuilderError> {
        let vertex = StrokeVertexConstructor::new_vertex(&mut (), vertex);
        self.mesh.vertices.push(vertex);
        Ok(VertexId(self.mesh.vertices.len() as u32 - 1))
    }
}

impl FillVertexConstructor<AbstractVertex> for () {
    fn new_vertex(&mut self, vertex: FillVertex) -> AbstractVertex {
        let position = Vector::new(vertex.position().x, vertex.position().y);
        AbstractVertex::new(position)
    }
}

impl StrokeVertexConstructor<AbstractVertex> for () {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> AbstractVertex {
        let position = Vector::new(vertex.position().x, vertex.position().y);
        AbstractVertex::new(position)
    }
}
