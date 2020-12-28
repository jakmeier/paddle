use crate::graphics::Image;
use crate::quicksilver_compat::graphics::Color;

/// The background to use for a given drawable
///
/// While each Drawable can define its own behavior, the recommended behavior
/// is that the Image be applied in proportion to the relative position of
/// the vertices. This means the left-most vertex should use the left edge
/// of the image, the right-most vertex should use the right edge of the image, etc.
#[derive(Copy, Clone)]
pub enum Background<'a> {
    /// A uniform color background
    Col(Color),
    /// A textured background
    Img(&'a Image),
    /// A color and image blended multiplicatively
    Blended(&'a Image, Color),
}

impl<'a> Background<'a> {
    /// Return either the stored Image or None
    pub fn image(&self) -> Option<&Image> {
        match self {
            Background::Col(_) => None,
            Background::Img(img) | Background::Blended(img, _) => Some(img),
        }
    }

    /// Return either the stored Color or Color::WHITE
    pub fn color(&self) -> Color {
        match self {
            Background::Col(color) | Background::Blended(_, color) => *color,
            Background::Img(_) => Color::WHITE,
        }
    }
}

impl<'a> From<Color> for Background<'a> {
    fn from(col: Color) -> Self {
        Background::Col(col)
    }
}

impl<'a> From<&'a Image> for Background<'a> {
    fn from(img: &'a Image) -> Self {
        Background::Img(img)
    }
}

impl<'a> From<(&'a Image, Color)> for Background<'a> {
    fn from((img, col): (&'a Image, Color)) -> Self {
        Background::Blended(img, col)
    }
}
