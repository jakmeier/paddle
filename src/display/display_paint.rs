use crate::UniformList;
use crate::{
    quicksilver_compat::Color, AbstractVertex, AssetLibrary, Image, Paint, RenderPipelineHandle,
};

/// Implementor of this trait can be used on `Display` and `DisplayArea` to fill geometric shapes.
///
/// This trait is similar to `Paint`. But it also has access the asset library on the display.
pub trait DisplayPaint {
    fn image<'a>(&'a self, _assets: &'a AssetLibrary) -> Option<&'a Image> {
        None
    }
    fn color(&self, _assets: &AssetLibrary) -> Option<Color> {
        None
    }
    fn extra_vertex_attributes(
        &self,
        _assets: &AssetLibrary,
        _index: usize,
        _vertex: &AbstractVertex,
    ) -> Option<Vec<f32>> {
        None
    }
    fn paint_uniforms(&self) -> UniformList {
        UniformList::default()
    }
    fn render_pipeline(&self, _assets: &AssetLibrary) -> RenderPipelineHandle {
        RenderPipelineHandle::default()
    }
}

/// A tuple of an asset library with anything that implements `DisplayPaint` also implements `Paint`.
/// This allows descriptors (and other display paints) to be used for rendering.
impl<DP: DisplayPaint> Paint for (&DP, &AssetLibrary) {
    fn paint_image(&self) -> Option<&Image> {
        DisplayPaint::image(self.0, &self.1)
    }
    fn paint_color(&self) -> Option<Color> {
        DisplayPaint::color(self.0, &self.1)
    }
    fn paint_extra_vertex_attributes(
        &self,
        index: usize,
        vertex: &AbstractVertex,
    ) -> Option<Vec<f32>> {
        DisplayPaint::extra_vertex_attributes(self.0, &self.1, index, vertex)
    }
    fn paint_uniforms(&self) -> UniformList {
        DisplayPaint::paint_uniforms(self.0)
    }
    fn paint_render_pipeline(&self) -> RenderPipelineHandle {
        DisplayPaint::render_pipeline(self.0, &self.1)
    }
}

// Anything that implements `Paint` also implements `DisplayPaint`.
// This allows normal paints to be used on the display draw methods.
impl<P: Paint> DisplayPaint for P {
    fn image<'a>(&'a self, _assets: &'a AssetLibrary) -> Option<&'a Image> {
        Paint::paint_image(self)
    }
    fn color(&self, _assets: &AssetLibrary) -> Option<Color> {
        Paint::paint_color(self)
    }
    fn extra_vertex_attributes(
        &self,
        _assets: &AssetLibrary,
        index: usize,
        vertex: &AbstractVertex,
    ) -> Option<Vec<f32>> {
        Paint::paint_extra_vertex_attributes(self, index, vertex)
    }
    fn paint_uniforms(&self) -> UniformList {
        Paint::paint_uniforms(self)
    }
    fn render_pipeline(&self, _assets: &AssetLibrary) -> RenderPipelineHandle {
        Paint::paint_render_pipeline(self)
    }
}

// Enable `Box<DisplayPaint>`` to be used as a generic store for something that can be drawn.
//
// Note: This might be removed in the future in favour of a strong type that serves the same purpose.
// Details unclear atm.
impl DisplayPaint for Box<dyn DisplayPaint> {
    fn image<'a>(&'a self, assets: &'a AssetLibrary) -> Option<&'a Image> {
        self.as_ref().image(assets)
    }
    fn color(&self, assets: &AssetLibrary) -> Option<Color> {
        self.as_ref().color(assets)
    }
    fn extra_vertex_attributes(
        &self,
        assets: &AssetLibrary,
        index: usize,
        vertex: &AbstractVertex,
    ) -> Option<Vec<f32>> {
        self.as_ref().extra_vertex_attributes(assets, index, vertex)
    }
    fn paint_uniforms(&self) -> UniformList {
        self.as_ref().paint_uniforms()
    }
    fn render_pipeline(&self, assets: &AssetLibrary) -> RenderPipelineHandle {
        self.as_ref().render_pipeline(assets)
    }
}
