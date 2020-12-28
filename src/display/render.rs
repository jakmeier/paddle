use crate::{AbstractMesh, GpuMesh, GpuTriangle, GpuVertex, Transform, Z_MAX};

/// Some object that can be rendered to a GpuMesh with only the position as parameter
pub trait Render {
    fn render(&self, mesh: &mut GpuMesh, transform: Transform, z: i16);
}

impl Render for AbstractMesh {
    fn render(&self, gpu_mesh: &mut GpuMesh, transform: Transform, z: i16) {
        let z = z as f32 / Z_MAX as f32;
        let n = gpu_mesh.vertices.len() as u32;
        for abstract_vertex in &self.vertices {
            gpu_mesh.vertices.push(GpuVertex {
                pos: transform * abstract_vertex.pos,
                tex_pos: abstract_vertex.tex_pos.map(|v| v * transform ),
                z,
                col: abstract_vertex.col,
            });
        }
        gpu_mesh.triangles.extend(
            self.triangles
                .iter()
                .map(|t| GpuTriangle::from_abstract(t, n, z)),
        );
    }
}
