use super::*;

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

/// 2-dimensional vector with abscissa and ordinate.
pub type Vector = Point<f32>;

impl From<[f32; 2]> for Vector {
    fn from(elements: [f32; 2]) -> Self {
        Self {
            x: elements[0],
            y: elements[1],
        }
    }
}

impl From<Vector> for [f32; 2] {
    fn from(vector: Vector) -> Self {
        [vector.x, vector.y]
    }
}

impl Add<f32> for Vector {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl AddAssign<f32> for Vector {
    fn add_assign(&mut self, other: f32) {
        *self = *self + other;
    }
}

impl Sub<f32> for Vector {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl SubAssign<f32> for Vector {
    fn sub_assign(&mut self, other: f32) {
        *self = *self - other;
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}
