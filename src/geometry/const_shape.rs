#![cfg(feature = "const_fn")]

use crate::geometry::{Rectangle, Vector};
use crate::quicksilver_compat::geom::{Circle, Line, Triangle};

impl Circle {
    pub const fn const_translate(&self, v: Vector) -> Self {
        Circle {
            pos: self.pos.const_translate(v),
            radius: self.radius,
        }
    }
}

impl Rectangle {
    pub const fn const_translate(&self, v: Vector) -> Self {
        Rectangle {
            pos: self.pos.const_translate(v),
            size: self.size,
        }
    }
    pub const fn const_center(&self) -> Vector {
        self.pos.const_translate(self.size.div(2.0))
    }
}

impl Triangle {
    pub const fn const_translate(&self, v: Vector) -> Self {
        let v = v;
        Triangle {
            a: self.a.const_translate(v),
            b: self.b.const_translate(v),
            c: self.c.const_translate(v),
        }
    }
    pub const fn const_center(&self) -> Vector {
        (self.a.const_translate(self.b.const_translate(self.c))).div(3.0)
    }
}

impl Line {
    pub const fn const_translate(&self, v: Vector) -> Self {
        let v = v;
        Line {
            a: self.a.const_translate(v),
            b: self.b.const_translate(v),
            t: self.t,
        }
    }
    pub const fn const_center(&self) -> Vector {
        (self.a.const_translate(self.b)).div(2.0)
    }
}

impl Vector {
    pub const fn const_translate(&self, v: Vector) -> Self {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
    pub const fn mul(&self, rhs: f32) -> Self {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
    pub const fn div(&self, rhs: f32) -> Self {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
