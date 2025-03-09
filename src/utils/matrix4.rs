use std::ops::{Add, Mul, Div, Sub, Neg, Deref, DerefMut};
use super::Vec4; // Assuming you have a Vec4 class similar to Vec3

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix4 {
    m: [[f64; 4]; 4],
}

impl Matrix4 {
    // Identity matrix
    pub const I: Self = Self {
        m: [[1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]]
    };

    #[inline]
    #[must_use]
    pub const fn new(m: [[f64; 4]; 4]) -> Self { Self { m } }
}

// Matrix + Matrix
impl Add<Self> for Matrix4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            m: [[self.m[0][0] + rhs.m[0][0], self.m[0][1] + rhs.m[0][1], self.m[0][2] + rhs.m[0][2], self.m[0][3] + rhs.m[0][3]],
                [self.m[1][0] + rhs.m[1][0], self.m[1][1] + rhs.m[1][1], self.m[1][2] + rhs.m[1][2], self.m[1][3] + rhs.m[1][3]],
                [self.m[2][0] + rhs.m[2][0], self.m[2][1] + rhs.m[2][1], self.m[2][2] + rhs.m[2][2], self.m[2][3] + rhs.m[2][3]],
                [self.m[3][0] + rhs.m[3][0], self.m[3][1] + rhs.m[3][1], self.m[3][2] + rhs.m[3][2], self.m[3][3] + rhs.m[3][3]]]
        }
    }
}

// Matrix - Matrix
impl Sub<Self> for Matrix4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            m: [[self.m[0][0] - rhs.m[0][0], self.m[0][1] - rhs.m[0][1], self.m[0][2] - rhs.m[0][2], self.m[0][3] - rhs.m[0][3]],
                [self.m[1][0] - rhs.m[1][0], self.m[1][1] - rhs.m[1][1], self.m[1][2] - rhs.m[1][2], self.m[1][3] - rhs.m[1][3]],
                [self.m[2][0] - rhs.m[2][0], self.m[2][1] - rhs.m[2][1], self.m[2][2] - rhs.m[2][2], self.m[2][3] - rhs.m[2][3]],
                [self.m[3][0] - rhs.m[3][0], self.m[3][1] - rhs.m[3][1], self.m[3][2] - rhs.m[3][2], self.m[3][3] - rhs.m[3][3]]]
        }
    }
}

// Matrix multiplication
impl Mul<Self> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4 {
            m: [
                [
                    self.m[0][0] * rhs.m[0][0] + self.m[0][1] * rhs.m[1][0] + self.m[0][2] * rhs.m[2][0] + self.m[0][3] * rhs.m[3][0],
                    self.m[0][0] * rhs.m[0][1] + self.m[0][1] * rhs.m[1][1] + self.m[0][2] * rhs.m[2][1] + self.m[0][3] * rhs.m[3][1],
                    self.m[0][0] * rhs.m[0][2] + self.m[0][1] * rhs.m[1][2] + self.m[0][2] * rhs.m[2][2] + self.m[0][3] * rhs.m[3][2],
                    self.m[0][0] * rhs.m[0][3] + self.m[0][1] * rhs.m[1][3] + self.m[0][2] * rhs.m[2][3] + self.m[0][3] * rhs.m[3][3],
                ],
                [
                    self.m[1][0] * rhs.m[0][0] + self.m[1][1] * rhs.m[1][0] + self.m[1][2] * rhs.m[2][0] + self.m[1][3] * rhs.m[3][0],
                    self.m[1][0] * rhs.m[0][1] + self.m[1][1] * rhs.m[1][1] + self.m[1][2] * rhs.m[2][1] + self.m[1][3] * rhs.m[3][1],
                    self.m[1][0] * rhs.m[0][2] + self.m[1][1] * rhs.m[1][2] + self.m[1][2] * rhs.m[2][2] + self.m[1][3] * rhs.m[3][2],
                    self.m[1][0] * rhs.m[0][3] + self.m[1][1] * rhs.m[1][3] + self.m[1][2] * rhs.m[2][3] + self.m[1][3] * rhs.m[3][3],
                ],
                [
                    self.m[2][0] * rhs.m[0][0] + self.m[2][1] * rhs.m[1][0] + self.m[2][2] * rhs.m[2][0] + self.m[2][3] * rhs.m[3][0],
                    self.m[2][0] * rhs.m[0][1] + self.m[2][1] * rhs.m[1][1] + self.m[2][2] * rhs.m[2][1] + self.m[2][3] * rhs.m[3][1],
                    self.m[2][0] * rhs.m[0][2] + self.m[2][1] * rhs.m[1][2] + self.m[2][2] * rhs.m[2][2] + self.m[2][3] * rhs.m[3][2],
                    self.m[2][0] * rhs.m[0][3] + self.m[2][1] * rhs.m[1][3] + self.m[2][2] * rhs.m[2][3] + self.m[2][3] * rhs.m[3][3],
                ],
                [
                    self.m[3][0] * rhs.m[0][0] + self.m[3][1] * rhs.m[1][0] + self.m[3][2] * rhs.m[2][0] + self.m[3][3] * rhs.m[3][0],
                    self.m[3][0] * rhs.m[0][1] + self.m[3][1] * rhs.m[1][1] + self.m[3][2] * rhs.m[2][1] + self.m[3][3] * rhs.m[3][1],
                    self.m[3][0] * rhs.m[0][2] + self.m[3][1] * rhs.m[1][2] + self.m[3][2] * rhs.m[2][2] + self.m[3][3] * rhs.m[3][2],
                    self.m[3][0] * rhs.m[0][3] + self.m[3][1] * rhs.m[1][3] + self.m[3][2] * rhs.m[2][3] + self.m[3][3] * rhs.m[3][3],
                ]
            ]
        }
    }
}

// Matrix4 * Vec4
impl Mul<Vec4> for Matrix4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            rhs.x * self.m[0][0] + rhs.y * self.m[0][1] + rhs.z * self.m[0][2] + rhs.w * self.m[0][3],
            rhs.x * self.m[1][0] + rhs.y * self.m[1][1] + rhs.z * self.m[1][2] + rhs.w * self.m[1][3],
            rhs.x * self.m[2][0] + rhs.y * self.m[2][1] + rhs.z * self.m[2][2] + rhs.w * self.m[2][3],
            rhs.x * self.m[3][0] + rhs.y * self.m[3][1] + rhs.z * self.m[3][2] + rhs.w * self.m[3][3],
        )
    }
}

// Matrix * f64
impl Mul<f64> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            m: [
                [self.m[0][0] * rhs, self.m[0][1] * rhs, self.m[0][2] * rhs, self.m[0][3] * rhs],
                [self.m[1][0] * rhs, self.m[1][1] * rhs, self.m[1][2] * rhs, self.m[1][3] * rhs],
                [self.m[2][0] * rhs, self.m[2][1] * rhs, self.m[2][2] * rhs, self.m[2][3] * rhs],
                [self.m[3][0] * rhs, self.m[3][1] * rhs, self.m[3][2] * rhs, self.m[3][3] * rhs],
            ]
        }
    }
}

// Matrix / f64
impl Div<f64> for Matrix4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            m: [
                [self.m[0][0] / rhs, self.m[0][1] / rhs, self.m[0][2] / rhs, self.m[0][3] / rhs],
                [self.m[1][0] / rhs, self.m[1][1] / rhs, self.m[1][2] / rhs, self.m[1][3] / rhs],
                [self.m[2][0] / rhs, self.m[2][1] / rhs, self.m[2][2] / rhs, self.m[2][3] / rhs],
                [self.m[3][0] / rhs, self.m[3][1] / rhs, self.m[3][2] / rhs, self.m[3][3] / rhs],
            ]
        }
    }
}

// -Matrix
impl Neg for Matrix4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            m: [
                [-self.m[0][0], -self.m[0][1], -self.m[0][2], -self.m[0][3]],
                [-self.m[1][0], -self.m[1][1], -self.m[1][2], -self.m[1][3]],
                [-self.m[2][0], -self.m[2][1], -self.m[2][2], -self.m[2][3]],
                [-self.m[3][0], -self.m[3][1], -self.m[3][2], -self.m[3][3]],
            ]
        }
    }
}

// Matrix[l]
impl Deref for Matrix4 {
    type Target = [[f64; 4]; 4];

    fn deref(&self) -> &Self::Target {
        &self.m
    }
}

// Implement DerefMut for Matrix4
impl DerefMut for Matrix4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.m
    }
}