use crate::quicksilver_compat::graphics::Color;
use crate::{graphics::Image, AbstractVertex, RenderPipelineHandle};

/// Implementors of this trait can be used to define non-positional attributes of GPU vertices. (Color/texture/custom attributes)
pub trait Paint {
    fn paint_image(&self) -> Option<&Image> {
        None
    }
    fn paint_color(&self) -> Option<Color> {
        None
    }
    fn paint_extra_vertex_attributes(&self, _index: usize, _vertex: &AbstractVertex) -> Option<Vec<f32>> {
        None
    }
    fn paint_render_pipeline(&self) -> RenderPipelineHandle {
        RenderPipelineHandle::default()
    }
}

impl Paint for Image {
    fn paint_image(&self) -> Option<&Image> {
        Some(&self)
    }
}
impl Paint for Color {
    fn paint_color(&self) -> Option<Color> {
        Some(*self)
    }
}
