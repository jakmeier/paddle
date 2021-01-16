use crate::graphics::Image;
use crate::quicksilver_compat::graphics::Color;

/// Defines the color coating for a shape. Can be backed by an image (Texture2D) or a color
#[derive(Copy, Clone)]
pub enum Paint<'a> {
    /// A uniform color background
    Col(Color),
    /// A textured background
    Img(&'a Image),
}

impl<'a> Paint<'a> {
    pub fn image(&self) -> Option<&Image> {
        match self {
            Paint::Col(_) => None,
            Paint::Img(img) => Some(img),
        }
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            Paint::Col(color) => Some(*color),
            Paint::Img(_) => None,
        }
    }
}

impl<'a> From<Color> for Paint<'a> {
    fn from(col: Color) -> Self {
        Paint::Col(col)
    }
}

impl<'a> From<&'a Image> for Paint<'a> {
    fn from(img: &'a Image) -> Self {
        Paint::Img(img)
    }
}
