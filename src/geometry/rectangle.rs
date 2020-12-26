use crate::geometry::{grid::Grid, Vector};
use crate::quicksilver_compat::geom::Shape;
use crate::{quicksilver_compat::about_equal, FitStrategy};
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A rectangle with a top-left position and a size
pub struct Rectangle {
    ///The top-left coordinate of the rectangle
    pub pos: Vector,
    ///The width and height of the rectangle
    pub size: Vector,
}

impl Rectangle {
    ///Create a rectangle from a top-left vector and a size vector
    pub fn new(pos: impl Into<Vector>, size: impl Into<Vector>) -> Rectangle {
        Rectangle {
            pos: pos.into(),
            size: size.into(),
        }
    }

    ///Create a rectangle at the origin with the given size
    pub fn new_sized(size: impl Into<Vector>) -> Rectangle {
        Rectangle {
            pos: Vector::ZERO,
            size: size.into(),
        }
    }

    ///Get the top left coordinate of the Rectangle
    pub fn top_left(&self) -> Vector {
        self.pos
    }

    ///Get the x-coordinate of the Rectangle
    ///(The origin of a Rectangle is at the top left)
    pub fn x(&self) -> f32 {
        self.pos.x
    }

    ///Get the y-coordinate of the Rectangle
    ///(The origin of a Rectangle is at the top left)
    pub fn y(&self) -> f32 {
        self.pos.y
    }

    ///Get the size of the Rectangle
    pub fn size(&self) -> Vector {
        self.size
    }

    ///Get the height of the Rectangle
    pub fn height(&self) -> f32 {
        self.size.y
    }

    ///Get the width of the Rectangle
    pub fn width(&self) -> f32 {
        self.size.x
    }

    #[must_use]
    pub fn shrink_to_center(&self, shrink_to_center: f32) -> Rectangle {
        Rectangle::new_sized(self.size() * shrink_to_center).with_center(self.center())
    }
    /// Padds constant pixels around the rectangle
    #[must_use]
    pub fn padded(&self, padding: f32) -> Rectangle {
        Rectangle::new_sized(self.size() - (2.0 * padding, 2.0 * padding).into())
            .with_center(self.center())
    }
    #[must_use]
    pub fn shrink_and_fit_into(self, frame: &Rectangle, fit_strat: FitStrategy) -> Rectangle {
        self.fit_into_ex(frame, fit_strat, false)
    }
    #[must_use]
    /// Shrinks (or grows) and moves the rectangle to fit within the given frame, without changing proportions
    pub fn fit_into(&self, frame: &Rectangle, fit_strat: FitStrategy) -> Rectangle {
        self.fit_into_ex(frame, fit_strat, true)
    }
    /// Shrinks and moves the rectangle to fit within the given frame, without changing proportions
    #[must_use]
    pub fn fit_into_ex(
        mut self,
        frame: &Rectangle,
        fit_strat: FitStrategy,
        allow_grow: bool,
    ) -> Rectangle {
        let stretch_factor = (frame.width() / self.width()).min(frame.height() / self.height());
        if allow_grow || stretch_factor < 1.0 {
            self.size *= stretch_factor;
        }
        match fit_strat {
            FitStrategy::TopLeft => self.pos = frame.pos,
            FitStrategy::LeftCenter => {
                self.pos = frame.pos + ((frame.size - self.size).y_comp() / 2.0)
            }
            FitStrategy::Center => {
                self.pos = frame.pos;
                self.pos = frame.pos + frame.center() - self.center()
            }
        }
        self
    }
    /// Finds the largest square that fits into the given rectangle
    #[must_use]
    pub fn fit_square(&self, fit_strat: FitStrategy) -> Rectangle {
        let s = self.width().min(self.height());
        let mut rect = Rectangle::new(self.pos, (s, s));
        match fit_strat {
            FitStrategy::Center => {
                rect = rect.translate(((self.width() - rect.width()) / 2.0, 0.0));
                rect = rect.translate((0.0, (self.height() - rect.height()) / 2.0));
            }
            FitStrategy::LeftCenter => {
                rect = rect.translate((0.0, (self.height() - rect.height()) / 2.0));
            }
            FitStrategy::TopLeft => {}
        }
        rect
    }

    pub fn grid(&self, cols: usize, rows: usize) -> Grid {
        let dx = self.width() / cols as f32;
        let dy = self.height() / rows as f32;
        Grid {
            base: Rectangle::new(self.pos, (dx, dy)),
            i: 0,
            x: cols,
            y: rows,
        }
    }
    #[must_use]
    pub fn cut_horizontal(&self, h: f32) -> (Rectangle, Rectangle) {
        let mut top = self.clone();
        top.size.y = h;
        let mut bottom = self.clone();
        bottom.size.y -= h;
        bottom.pos.y += h;
        (top, bottom)
    }
    #[must_use]
    pub fn cut_vertical(&self, w: f32) -> (Rectangle, Rectangle) {
        let mut left = self.clone();
        left.size.x = w;
        let mut right = self.clone();
        right.size.x -= w;
        right.pos.x += w;
        (left, right)
    }
}

#[cfg(feature = "const_fn")]
impl Rectangle {
    pub const fn const_shrink_to_center(&self, stretch_factor: f32) -> Rectangle {
        let size = Vector {
            x: self.size.x * stretch_factor,
            y: self.size.y * stretch_factor,
        };
        Rectangle {
            pos: self.const_center(),
            size,
        }
        .const_translate(Vector {
            x: -size.x / 2.0,
            y: -size.y / 2.0,
        })
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Rectangle) -> bool {
        about_equal(self.x(), other.pos.x)
            && about_equal(self.y(), other.pos.y)
            && about_equal(self.width(), other.size.x)
            && about_equal(self.height(), other.size.y)
    }
}

impl Eq for Rectangle {}

#[cfg(test)]
mod tests {
    use crate::{quicksilver_compat::*, Rectangle, Vector};

    #[test]
    fn overlap() {
        let a = &Rectangle::new_sized((32, 32));
        let b = &Rectangle::new((16, 16), (32, 32));
        let c = &Rectangle::new((50, 50), (5, 5));
        assert!(a.overlaps(b));
        assert!(!a.overlaps(c));
    }

    #[test]
    fn contains() {
        let rect = Rectangle::new_sized((32, 32));
        let vec1 = Vector::new(5, 5);
        let vec2 = Vector::new(33, 1);
        assert!(rect.contains(vec1));
        assert!(!rect.contains(vec2));
    }

    #[test]
    fn constraint() {
        let constraint = &Rectangle::new_sized((10, 10));
        let a = Rectangle::new((-1, 3), (5, 5));
        let b = Rectangle::new((4, 4), (8, 3));
        let a = a.constrain(constraint);
        assert_eq!(a.top_left(), Vector::new(0, 3));
        let b = b.constrain(constraint);
        assert_eq!(b.top_left(), Vector::new(2, 4));
    }

    #[test]
    fn translate() {
        let a = Rectangle::new((10, 10), (5, 5));
        let v = Vector::new(1, -1);
        let translated = a.translate(v);
        assert_eq!(a.top_left() + v, translated.top_left());
    }
}
