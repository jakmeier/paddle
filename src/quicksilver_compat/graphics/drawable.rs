//! TODO: What's left in here needs to be cleaned up.
//! This extends to the general draw interface, where the separation between tesselation and rendering is not well represented now.

use crate::quicksilver_compat::graphics::Color;
use crate::{graphics::Image, Transform};

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
    /// A view into an image with transformation
    ImgView(&'a Image, Transform),
    /// A color and image blended multiplicatively
    Blended(&'a Image, Color),
}

impl<'a> Background<'a> {
    /// Return either the stored Image or None
    pub fn image(&self) -> Option<&Image> {
        match self {
            Background::Col(_) => None,
            Background::Img(img) | Background::Blended(img, _) | Background::ImgView(img, _) => {
                Some(img)
            }
        }
    }

    /// Return either the stored Color or Color::WHITE
    pub fn color(&self) -> Color {
        match self {
            Background::Col(color) | Background::Blended(_, color) => *color,
            Background::Img(_) | Background::ImgView(_, _) => Color::WHITE,
        }
    }

    /// Transformation to be applied to the texture (in normalized texture coordinates)
    pub fn texture_transform(&self) -> Option<Transform> {
        match self {
            Background::Col(_) | Background::Blended(_, _) => None,
            Background::Img(img) => Some(img.texture_transform()),
            Background::ImgView(img, transform) => Some(img.texture_transform() * *transform),
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
