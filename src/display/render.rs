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
        paint: impl Into<Paint<'a>>,
        z: i16,
    );
}

impl Render for AbstractMesh {
    fn render<'a>(
        &self,
        gpu_mesh: &mut GpuMesh,
        area: Rectangle,
        transform: Transform,
        paint: impl Into<Paint<'a>>,
        z: i16,
    ) {
        let paint = paint.into();
        let z = z as f32 / Z_MAX as f32;
        let n = gpu_mesh.vertices.len() as u32;
        let abstract_space = Rectangle::new((-1, -1), (2, 2));
        let position_transform = abstract_space.project(&area);
        for abstract_vertex in &self.vertices {
            let pos = transform * position_transform * abstract_vertex.pos;
            let tex = paint
                .image()
                .map(|img| img.sample(&abstract_space, &abstract_vertex.pos));
            let col = paint.color().unwrap_or(Color::WHITE);
            gpu_mesh.vertices.push(GpuVertex::new(pos, tex, col, z));
        }
        gpu_mesh.triangles.extend(
            self.triangles
                .iter()
                .map(|t| GpuTriangle::from_abstract(t, n, z)),
        );
    }
}
