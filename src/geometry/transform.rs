use serde::{Deserialize, Serialize};

use crate::quicksilver_compat::about_equal;
use crate::{Scalar, Vector};
use std::{
    cmp::{Eq, PartialEq},
    default::Default,
    f32::consts::PI,
    fmt,
    ops::Mul,
};

/// A 2D transformation represented by a matrix
///
/// Transforms can be composed together through matrix multiplication, and are applied to Vectors
/// through multiplication, meaning the notation used is the '*' operator. A property of matrix
/// multiplication is that for some matrices A, B, C and vector V is
/// ```text
/// Transform = A * B * C
/// Transform * V = A * (B * (C * V))
/// ```
///
/// This property allows encoding multiple transformations in a single matrix. A transformation
/// that involves rotating a shape 30 degrees and then moving it six units up could be written as
/// ```no_run
/// use paddle::quicksilver_compat::{Transform, Vector};
/// let transform = Transform::rotate(30) * Transform::translate(Vector::new(0, -6));
/// ```
/// and then applied to a Vector
/// ```no_run
/// # use paddle::quicksilver_compat::{Transform, Vector};
/// # let transform  = Transform::rotate(30) * Transform::translate(Vector::new(0, -6));
/// transform * Vector::new(5, 5)
/// # ;
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Transform([f32; 9]);

impl Transform {
    ///The identity transformation
    pub const IDENTITY: Transform =
        Transform::from_array([[1f32, 0f32, 0f32], [0f32, 1f32, 0f32], [0f32, 0f32, 1f32]]);

    ///Create a Transform from an arbitrary matrix in a column-major matrix
    pub const fn from_array(array: [[f32; 3]; 3]) -> Transform {
        Transform([
            array[0][0],
            array[0][1],
            array[0][2],
            array[1][0],
            array[1][1],
            array[1][2],
            array[2][0],
            array[2][1],
            array[2][2],
        ])
    }

    ///Create a rotation transformation
    pub fn rotate<T: Scalar>(angle: T) -> Transform {
        let angle = angle.float();
        let c = (angle * PI / 180f32).cos();
        let s = (angle * PI / 180f32).sin();
        Transform::from_array([[c, -s, 0f32], [s, c, 0f32], [0f32, 0f32, 1f32]])
    }

    ///Create a translation transformation
    pub fn translate(vec: impl Into<Vector>) -> Transform {
        let vec = vec.into();
        Transform::from_array([[1f32, 0f32, vec.x], [0f32, 1f32, vec.y], [0f32, 0f32, 1f32]])
    }

    ///Create a scale transformation
    pub fn scale(vec: impl Into<Vector>) -> Transform {
        let vec = vec.into();
        Transform::from_array([[vec.x, 0f32, 0f32], [0f32, vec.y, 0f32], [0f32, 0f32, 1f32]])
    }

    pub fn horizontal_flip() -> Transform {
        Transform::from_array([[-1f32, 0f32, 0f32], [0f32, 1f32, 0f32], [0f32, 0f32, 1f32]])
    }

    pub fn vertical_flip() -> Transform {
        Transform::from_array([[1f32, 0f32, 0f32], [0f32, -1f32, 0f32], [0f32, 0f32, 1f32]])
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
    pub fn row_major(&self) -> Vec<f32> {
        vec![
            self.0[0], self.0[3], self.0[6], self.0[1], self.0[4], self.0[7], self.0[2], self.0[5],
            self.0[8],
        ]
    }

    ///Find the inverse of a Transform
    ///
    /// A transform's inverse will cancel it out when multplied with it, as seen below:
    /// ```
    /// # use paddle::quicksilver_compat::{Transform, Vector};
    /// let transform = Transform::translate(Vector::new(4, 5));
    /// let inverse = transform.inverse();
    /// let vector = Vector::new(10, 10);
    /// assert_eq!(vector, transform * inverse * vector);
    /// assert_eq!(vector, inverse * transform * vector);
    /// ```
    #[must_use]
    pub fn inverse(&self) -> Transform {
        let det = self.0[0] * (self.0[4] * self.0[8] - self.0[7] * self.0[5])
            - self.0[1] * (self.0[3] * self.0[8] - self.0[5] * self.0[6])
            + self.0[2] * (self.0[3] * self.0[7] - self.0[4] * self.0[6]);

        let inv_det = det.recip();

        let mut inverse = Transform::IDENTITY;
        inverse.0[0] = self.0[4] * self.0[8] - self.0[7] * self.0[5];
        inverse.0[1] = self.0[2] * self.0[7] - self.0[1] * self.0[8];
        inverse.0[2] = self.0[1] * self.0[5] - self.0[2] * self.0[4];
        inverse.0[3] = self.0[5] * self.0[6] - self.0[3] * self.0[8];
        inverse.0[4] = self.0[0] * self.0[8] - self.0[2] * self.0[6];
        inverse.0[5] = self.0[3] * self.0[2] - self.0[0] * self.0[5];
        inverse.0[6] = self.0[3] * self.0[7] - self.0[6] * self.0[4];
        inverse.0[7] = self.0[6] * self.0[1] - self.0[0] * self.0[7];
        inverse.0[8] = self.0[0] * self.0[4] - self.0[3] * self.0[1];
        inverse * inv_det
    }
}

///Concat two transforms A and B such that A * B * v = A * (B * v)
impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        let mut returnval = Transform::IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                returnval.0[i * 3 + j] = 0f32;
                for k in 0..3 {
                    returnval.0[i * 3 + j] += other.0[k * 3 + j] * self.0[i * 3 + k];
                }
            }
        }
        returnval
    }
}

///Transform a vector
impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector::new(
            other.x * self.0[0] + other.y * self.0[1] + self.0[2],
            other.x * self.0[3] + other.y * self.0[4] + self.0[5],
        )
    }
}
impl Mul<Transform> for Vector {
    type Output = Vector;

    fn mul(self, t: Transform) -> Vector {
        Vector::new(
            self.x * t.0[0] + self.y * t.0[3] + t.0[6],
            self.x * t.0[1] + self.y * t.0[4] + t.0[7],
        )
    }
}

/// Scale all of the internal values of the Transform matrix
///
/// Note this will NOT scale vectors multiplied by this transform, and generally you shouldn't need
/// to use this.
impl<T: Scalar> Mul<T> for Transform {
    type Output = Transform;

    fn mul(self, other: T) -> Transform {
        let other = other.float();
        let mut ret = Transform::IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                ret.0[i * 3 + j] = self.0[i * 3 + j] * other;
            }
        }
        ret
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..3 {
            for j in 0..3 {
                write!(f, "{},", self.0[i * 3 + j])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::IDENTITY
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Transform) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !about_equal(self.0[i * 3 + j], other.0[i * 3 + j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Transform {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Transform::IDENTITY, Transform::IDENTITY);
        assert_eq!(Transform::rotate(5), Transform::rotate(5));
    }

    #[test]
    fn inverse() {
        let vec = Vector::new(2, 4);
        let translate = Transform::scale(Vector::ONE * 0.5);
        let inverse = translate.inverse();
        let transformed = inverse * vec;
        let expected = vec * 2;
        assert_eq!(transformed, expected);
    }

    #[test]
    fn scale() {
        let trans = Transform::scale(Vector::ONE * 2);
        let vec = Vector::new(2, 5);
        let scaled = trans * vec;
        let expected = vec * 2;
        assert_eq!(scaled, expected);
    }

    #[test]
    fn translate() {
        let translate = Vector::new(3, 4);
        let trans = Transform::translate(translate);
        let vec = Vector::ONE;
        let translated = trans * vec;
        let expected = vec + translate;
        assert_eq!(translated, expected);
    }

    #[test]
    fn identity() {
        let trans = Transform::IDENTITY
            * Transform::translate(Vector::ZERO)
            * Transform::rotate(0f32)
            * Transform::scale(Vector::ONE);
        let vec = Vector::new(15, 12);
        assert_eq!(vec, trans * vec);
    }

    #[test]
    fn complex_inverse() {
        let a = Transform::rotate(5f32)
            * Transform::scale(Vector::new(0.2, 1.23))
            * Transform::translate(Vector::ONE * 100f32);
        let a_inv = a.inverse();
        let vec = Vector::new(120f32, 151f32);
        assert_eq!(vec, a * a_inv * vec);
        assert_eq!(vec, a_inv * a * vec);
    }
}
