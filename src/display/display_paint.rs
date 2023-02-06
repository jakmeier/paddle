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
    fn render_pipeline(&self, _assets: &AssetLibrary) -> RenderPipelineHandle {
        Paint::paint_render_pipeline(self)
    }
}
