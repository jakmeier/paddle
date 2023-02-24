use crate::Rectangle;
use crate::{
    quicksilver_compat::Color, AbstractMesh, GpuMesh, GpuTriangle, GpuVertex, Paint, Transform,
    Z_MAX,
};

use super::ABSTRACT_SPACE;

/// Some object that can be rendered to a GpuMesh with position parameters and a color or image
pub trait Render {
    fn render<'a>(&self, mesh: &mut GpuMesh, transform: &Transform, paint: &impl Paint, z: i16);
}

impl Render for AbstractMesh {
    fn render<'a>(
        &self,
        gpu_mesh: &mut GpuMesh,
        transform: &Transform,
        paint: &impl Paint,
        z: i16,
    ) {
        let z = z as f32 / Z_MAX as f32;
        let n = gpu_mesh.vertices.len() as u32;
        let col = paint.paint_color().unwrap_or(Color::WHITE);

        for (index, abstract_vertex) in self.vertices.iter().enumerate() {
            let pos = *transform * abstract_vertex.pos;
            let region = paint
                .paint_image()
                .map(|img| img.region)
                .unwrap_or(Rectangle::new_sized((1, 1)));
            let st = super::gpu::sample(&ABSTRACT_SPACE, &abstract_vertex.pos, &region);
            let tex = paint
                .paint_image()
                .map(|img| img.texture.webgl_texture().clone());
            let extra = paint.paint_extra_vertex_attributes(index, abstract_vertex);
            gpu_mesh
                .vertices
                .push(GpuVertex::new(pos, tex, st, col, z, extra));
        }
        gpu_mesh.triangles.extend(
            self.triangles
                .iter()
                .map(|t| GpuTriangle::from_abstract(t, n, z)),
        );
    }
}
