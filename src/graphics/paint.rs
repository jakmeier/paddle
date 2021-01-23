use crate::quicksilver_compat::graphics::Color;
use crate::{graphics::Image, AbstractVertex, RenderPipelineHandle};

/// Implementors of this trait can be used to define non-positional attributes of GPU vertices. (Color/texture/custom attributes)
pub trait Paint {
    fn image(&self) -> Option<&Image> {
        None
    }
    fn color(&self) -> Option<Color> {
        None
    }
    fn extra_vertex_attributes(&self, _index: usize, _vertex: &AbstractVertex) -> Option<Vec<f32>> {
        None
    }
    fn render_pipeline(&self) -> RenderPipelineHandle {
        RenderPipelineHandle::default()
    }
}

impl Paint for Image {
    fn image(&self) -> Option<&Image> {
        Some(&self)
    }
}
impl Paint for Color {
    fn color(&self) -> Option<Color> {
        Some(*self)
    }
}
