use crate::{
    quicksilver_compat::Color, AbstractMesh, GpuMesh, GpuTriangle, GpuVertex, Paint, Rectangle,
    Transform, Z_MAX,
};

/// Some object that can be rendered to a GpuMesh with position parameters and a color or image
pub trait Render {
    fn render<'a>(
        &self,
        mesh: &mut GpuMesh,
        area: Rectangle,
        transform: Transform,
        paint: &impl Paint,
        z: i16,
    );
}

impl Render for AbstractMesh {
    fn render<'a>(
        &self,
        gpu_mesh: &mut GpuMesh,
        area: Rectangle,
        transform: Transform,
        paint: &impl Paint,
        z: i16,
    ) {
        let z = z as f32 / Z_MAX as f32;
        let n = gpu_mesh.vertices.len() as u32;
        let abstract_space = Rectangle::new((-1, -1), (2, 2));
        let position_transform = abstract_space.project(&area);
        let col = paint.color().unwrap_or(Color::WHITE);
        for (index, abstract_vertex) in self.vertices.iter().enumerate() {
            let pos = transform * position_transform * abstract_vertex.pos;
            let tex = paint
                .image()
                .map(|img| img.sample(&abstract_space, &abstract_vertex.pos));
            let extra = paint.extra_vertex_attributes(index, abstract_vertex);
            gpu_mesh
                .vertices
                .push(GpuVertex::new(pos, tex, col, z, extra));
        }
        gpu_mesh.triangles.extend(
            self.triangles
                .iter()
                .map(|t| GpuTriangle::from_abstract(t, n, z)),
        );
    }
}
