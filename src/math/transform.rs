use std::f32::consts::PI;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Sub, SubAssign,
};

use super::*;

/// The transformation matrix for 2 dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    elements: Elements,
}

type Elements = [[f32; 3]; 3];

impl Transform {
    /// Constructs the identity matrix.
    pub fn identity() -> Self {
        Self {
            elements: [
                [1f32, 0f32, 0f32],
                [0f32, 1f32, 0f32],
                [0f32, 0f32, 1f32],
            ],
        }
    }

    /// Constructs a transformation matrix where all the elements are equal to
    /// zero.
    pub fn zero() -> Self {
        Self {
            elements: [
                [0f32, 0f32, 0f32],
                [0f32, 0f32, 0f32],
                [0f32, 0f32, 0f32],
            ],
        }
    }

    /// Constructs a translation transformation and specifies the displacements
    /// in the direction of the x-axis and y-axis.
    pub fn translate(translation: impl Into<Vector>) -> Self {
        let translation = translation.into();
        Self {
            elements: [
                [1f32, 0f32, translation.x],
                [0f32, 1f32, translation.y],
                [0f32, 0f32, 1f32],
            ],
        }
    }

    /// Gets the translation vector.
    pub fn translation(&self) -> Vector {
        Vector {
            x: self[0][2],
            y: self[1][2],
        }
    }

    /// Constructs a scale transformation that has the specified scale factors
    /// and the origin as center point.
    pub fn scale(scale: impl Into<Vector>) -> Self {
        let scale = scale.into();
        Self {
            elements: [
                [scale.x, 0f32, 0f32],
                [0f32, scale.y, 0f32],
                [0f32, 0f32, 1f32],
            ],
        }
    }

    /// Constructs a scale transformation that has the specified scale factors
    /// and the given center point.
    pub fn scale_around(
        scale: impl Into<Vector>,
        center: impl Into<Coordinate>,
    ) -> Self {
        let scale = scale.into();
        let Coordinate { x, y } = center.into();
        Self {
            elements: [
                [scale.x, 0f32, (1f32 - scale.x) * x],
                [0f32, scale.y, (1f32 - scale.y) * y],
                [0f32, 0f32, 1f32],
            ],
        }
    }

    /// Gets the scaling vector.
    pub fn scaling(&self) -> Vector {
        Vector {
            x: (self[0][0].powf(2.0) + self[1][0].powf(2.0)).sqrt(),
            y: (self[0][1].powf(2.0) + self[1][1].powf(2.0)).sqrt(),
        }
    }

    /// Constructs a rotation transformation with the given angle in degrees
    /// around the origin.
    pub fn rotate(angle: f32) -> Self {
        let cosine = (angle * PI / 180f32).cos();
        let sine = (angle * PI / 180f32).sin();
        Self {
            elements: [
                [cosine, -sine, 0f32],
                [sine, cosine, 0f32],
                [0f32, 0f32, 1f32],
            ],
        }
    }

    /// Constructs a rotation transformation with the given angle in degrees and
    /// the given center point.
    pub fn rotate_around(angle: f32, center: impl Into<Coordinate>) -> Self {
        let c = (angle * PI / 180f32).cos();
        let s = (angle * PI / 180f32).sin();
        let Coordinate { x, y } = center.into();
        Self {
            elements: [
                [c, -s, -x * c + y * s + x],
                [s, c, -x * s - y * c + y],
                [0f32, 0f32, 1f32],
            ],
        }
    }

    /// Gets the rotation angle in degrees.
    pub fn rotation(&self) -> f32 {
        // compute the x skew angle
        180f32 / PI * self[1][1].atan2(self[0][1]) - 90f32
    }

    /// Gets the transpose of this matrix.
    pub fn transpose(&self) -> Self {
        let mut t = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                t[i][j] += self[j][i];
            }
        }
        t
    }

    /// Gets the 4x4 row matrix representation of this transformation matrix.
    pub fn to_row_matrix4(&self) -> [[f32; 4]; 4] {
        let mut matrix = [[0f32; 4]; 4];

        matrix[0][0] = self[0][0];
        matrix[0][1] = self[0][1];
        matrix[0][2] = 0f32;
        matrix[0][3] = self[0][2];

        matrix[1][0] = self[1][0];
        matrix[1][1] = self[1][1];
        matrix[1][2] = 0f32;
        matrix[1][3] = self[1][2];

        matrix[2][0] = 0f32;
        matrix[2][1] = 0f32;
        matrix[2][2] = 1f32;
        matrix[2][3] = 0f32;

        matrix[3][0] = self[2][0];
        matrix[3][1] = self[2][1];
        matrix[3][2] = 0f32;
        matrix[3][3] = self[2][2];

        matrix
    }

    /// Gets the 4x4 column matrix representation of this transformation matrix.
    pub fn to_column_matrix4(&self) -> [[f32; 4]; 4] {
        let mut matrix = [[0f32; 4]; 4];

        matrix[0][0] = self[0][0];
        matrix[0][1] = self[1][0];
        matrix[0][2] = 0f32;
        matrix[0][3] = self[2][0];

        matrix[1][0] = self[0][1];
        matrix[1][1] = self[1][1];
        matrix[1][2] = 0f32;
        matrix[1][3] = self[2][1];

        matrix[2][0] = 0f32;
        matrix[2][1] = 0f32;
        matrix[2][2] = 1f32;
        matrix[2][3] = 0f32;

        matrix[3][0] = self[0][2];
        matrix[3][1] = self[1][2];
        matrix[3][2] = 0f32;
        matrix[3][3] = self[2][2];

        matrix
    }
}

impl Default for Transform {
    /// Returns the identity matrix.
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Elements> for Transform {
    fn from(elements: Elements) -> Self {
        Self { elements }
    }
}

impl From<Transform> for Elements {
    fn from(transform: Transform) -> Self {
        transform.elements
    }
}

impl Deref for Transform {
    type Target = Elements;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl DerefMut for Transform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl Mul<Transform> for Transform {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut t = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    t[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        t
    }
}

impl MulAssign<Transform> for Transform {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: other.x * self[0][0] + other.y * self[0][1] + self[0][2],
            y: other.x * self[1][0] + other.y * self[1][1] + self[1][2],
        }
    }
}

impl Mul<f32> for Transform {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut t = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                t[i][j] = self[i][j] * other;
            }
        }
        t
    }
}

impl MulAssign<f32> for Transform {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl Add<Transform> for Transform {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut t = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                t[i][j] = other[i][j] + self[i][j];
            }
        }
        t
    }
}

impl AddAssign<Transform> for Transform {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub<Transform> for Transform {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut t = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                t[i][j] = self[i][j] - other[i][j];
            }
        }
        t
    }
}

impl SubAssign<Transform> for Transform {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
