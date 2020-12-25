use crate::{
    error::NutsCheck,
    quicksilver_compat::{
        geom::Scalar, Background, Drawable, Mesh, Rectangle, Shape, Transform, Vector,
    },
    Display, ErrorMessage,
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
    pub fn display_to_frame_coordinates(&self) -> Transform {
        Transform::translate(-self.region.pos)
    }
    /// In game coordinates (covering full display)
    pub fn is_inside(&self, display_coordinates: impl Into<Vector>) -> bool {
        self.region.contains(display_coordinates)
    }
    /// Draw a Drawable to the window, which will be finalized on the next flush
    pub fn draw<'a>(&'a mut self, draw: &impl Drawable, bkg: impl Into<Background<'a>>) {
        self.display
            .canvas
            .draw_ex(draw, bkg.into(), self.frame_to_display_coordinates(), 0.0);
    }
    /// Fills selected area with the given color (or image)
    pub fn fill<'a>(&'a mut self, bkg: impl Into<Background<'a>>) {
        self.display.canvas.draw(&self.region, bkg);
    }
    /// Draw a Drawable to the window with more options provided (draw exhaustive)
    pub fn draw_ex<'a>(
        &'a mut self,
        draw: &impl Drawable,
        bkg: impl Into<Background<'a>>,
        trans: Transform,
        z: impl Scalar,
    ) {
        self.display
            .canvas
            .draw_ex(draw, bkg, self.frame_to_display_coordinates() * trans, z)
    }
    /// Fit (the entire display) to be fully visible
    pub fn fit_display(&mut self, margin: f64) {
        self.display.fit_to_visible_area(margin).nuts_check();
    }
    /// Draw onto the display area from a mesh of triangles. Useful for custom tesselation.
    pub fn draw_triangles(&mut self, mesh: &Mesh) {
        let frame_transform = self.frame_to_display_coordinates();
        self.display.draw_triangles_ex(mesh, frame_transform);
    }
    /// Draw onto the display area from a mesh of triangles. The transformation will be applied to each triangle.
    pub fn draw_triangles_ex(&mut self, mesh: &Mesh, t: Transform) {
        let frame_transform = self.frame_to_display_coordinates();
        self.display.draw_triangles_ex(mesh, t * frame_transform);
    }
    pub fn add_html(&self, element: Element) {
        if let Some(parent) = self.div.parent_element().nuts_check() {
            parent
                .append_with_node_1(&element)
                .map_err(|e| ErrorMessage::technical(format!("Failed to add HTML: {:?}", e)))
                .nuts_check();
        }
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
