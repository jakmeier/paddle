use crate::{quicksilver_compat::Color, AbstractVertex, Image, Paint, RenderPipelineHandle};

pub struct CustomShader {
    pipe: RenderPipelineHandle,
    color: Option<Color>,
    image: Option<Image>,
}

impl CustomShader {
    pub fn new(pipe: RenderPipelineHandle) -> Self {
        Self {
            pipe,
            color: None,
            image: None,
        }
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn with_image(mut self, image: Image) -> Self {
        self.image = Some(image);
        self
    }
}

impl Paint for CustomShader {
    fn paint_extra_vertex_attributes(
        &self,
        _index: usize,
        _vertex: &AbstractVertex,
    ) -> Option<Vec<f32>> {
        None
        // TODO?
    }
    fn paint_render_pipeline(&self) -> RenderPipelineHandle {
        self.pipe
    }

    fn paint_image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    fn paint_color(&self) -> Option<Color> {
        self.color
    }
}
