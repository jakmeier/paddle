use crate::{
    error::NutsCheck, graphics::AbstractMesh, quicksilver_compat::Shape, Display, DisplayPaint,
    ErrorMessage, Rectangle, RenderPipelineHandle, Tessellate, Transform, Vector,
};
use div::DivHandle;
use web_sys::Element;

pub struct DisplayArea {
    /// in game coordinates (0|0 is at the top left of display)
    region: Rectangle,
    /// the full display
    display: Display,
    /// Div element that covers the display area, which is used for displaying HTML
    div: DivHandle,
}

impl DisplayArea {
    /// Select an area inside the full display. Ara specified in game coordinates.
    pub fn select(&mut self, rect: Rectangle, div: DivHandle) -> &mut Self {
        self.region = rect;
        self.div = div;
        self
    }
    /// The full display area.
    pub fn full(&self) -> &Display {
        &self.display
    }
    /// The full display area.
    pub fn full_mut(&mut self) -> &mut Display {
        &mut self.display
    }
    /// Converts from coordinates used inside the frame (where 0,0 is at the top left corner of the frame area)
    /// to a coordinate system covering the full display
    pub fn frame_to_display_coordinates(&self) -> Transform {
        Transform::translate(self.region.pos)
    }
    pub fn frame_to_display_area(&self, mut rect: Rectangle) -> Rectangle {
        rect.pos += self.region.pos;
        rect
    }
    pub fn frame_to_browser_area(&self, rect: Rectangle) -> Rectangle {
        self.full()
            .game_to_browser_area(self.frame_to_display_area(rect))
    }
    pub fn display_to_frame_coordinates(&self) -> Transform {
        Transform::translate(-self.region.pos)
    }
    /// In game coordinates (covering full display)
    pub fn is_inside(&self, display_coordinates: impl Into<Vector>) -> bool {
        self.region.contains(display_coordinates)
    }
    /// Draw something onto the window, which will be finalized on the next flush.
    ///
    /// This is the simplest draw function. It draws rectangular shapes and fills them with a paint.
    /// See `draw_ex` for more drawing options.
    pub fn draw(&mut self, position: &Rectangle, bkg: &impl DisplayPaint) {
        let trans = self.frame_to_display_coordinates();
        self.display.draw_ex(position, bkg, &trans, 0);
    }
    pub fn draw_z(&mut self, position: &Rectangle, bkg: &impl DisplayPaint, z: i16) {
        let trans = self.frame_to_display_coordinates();
        self.display.draw_ex(position, bkg, &trans, z);
    }
    /// Fills selected area with the given color (or image)
    pub fn fill(&mut self, bkg: &impl DisplayPaint) {
        let region = Rectangle::new_sized(self.region.size);
        self.draw(&region, bkg);
    }
    /// Draw a Drawable to the window with more options provided (draw exhaustive)
    pub fn draw_ex(
        &mut self,
        draw: &impl Tessellate,
        bkg: &impl DisplayPaint,
        trans: Transform,
        z: i16,
    ) {
        let trans = self.frame_to_display_coordinates() * trans;
        self.display.draw_ex(draw, bkg, &trans, z)
    }
    /// Fit (the entire display) to be fully visible
    pub fn fit_display(&mut self, margin: f64) {
        self.display.fit_to_visible_area(margin).nuts_check();
    }
    /// Draw onto the display area from a mesh of triangles. Useful for custom tesselation.
    pub fn draw_mesh<'a>(
        &mut self,
        mesh: &AbstractMesh,
        area: Rectangle,
        paint: &impl DisplayPaint,
    ) {
        let area = self.frame_to_display_area(area);
        self.display
            .draw_mesh_ex(mesh, paint, area, &Transform::IDENTITY, 0);
    }
    /// Draw onto the display area from a mesh of triangles. The transformation will be applied to each triangle.
    pub fn draw_mesh_ex<'a>(
        &mut self,
        mesh: &AbstractMesh,
        area: Rectangle,
        paint: &impl DisplayPaint,
        t: Transform,
        z: i16,
    ) {
        let area = self.frame_to_display_area(area);
        self.display.draw_mesh_ex(mesh, paint, area, &t, z);
    }
    pub fn add_html(&self, element: Element) {
        if let Some(parent) = self.div.parent_element().nuts_check() {
            parent
                .append_with_node_1(&element)
                .map_err(|e| ErrorMessage::technical(format!("Failed to add HTML: {:?}", e)))
                .nuts_check();
        }
    }
    /// The size of the selected area, in game coordinates
    pub fn size(&self) -> Vector {
        self.region.size()
    }

    // TODO: Find a better way to expose this
    pub fn new_render_pipeline(
        &mut self,
        vertex_shader_text: &'static str,
        fragment_shader_text: &'static str,
        vertex_descriptor: super::gpu::VertexDescriptor,
        uniform_values: &[(&'static str, super::gpu::UniformValue)],
    ) -> crate::PaddleResult<crate::RenderPipelineHandle> {
        self.full_mut().new_render_pipeline(
            vertex_shader_text,
            fragment_shader_text,
            vertex_descriptor,
            uniform_values,
        )
    }
    pub fn update_uniform(
        &mut self,
        rp: RenderPipelineHandle,
        name: &'static str,
        value: &super::gpu::UniformValue,
    ) {
        self.full_mut().update_uniform(rp, name, value)
    }
}

impl Into<DisplayArea> for Display {
    fn into(self) -> DisplayArea {
        DisplayArea {
            region: Rectangle::new_sized(self.game_coordinates),
            div: self.div.clone(),
            display: self,
        }
    }
}
