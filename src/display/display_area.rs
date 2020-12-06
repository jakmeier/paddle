use crate::{
    error::NutsCheck,
    quicksilver_compat::{geom::Scalar, Background, Drawable, Rectangle, Shape, Transform, Vector},
    Display,
};

pub struct DisplayArea {
    /// in game coordinates (0|0 is at the top left of display)
    region: Rectangle,
    /// the full display
    display: Display,
}

impl DisplayArea {
    /// Select an area inside the full display. Specified in game coordinates.
    pub fn select(&mut self, rect: Rectangle) -> &mut Self {
        self.region = rect;
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
}

impl Into<DisplayArea> for Display {
    fn into(self) -> DisplayArea {
        DisplayArea {
            region: Rectangle::new_sized(self.game_coordinates),
            display: self,
        }
    }
}
