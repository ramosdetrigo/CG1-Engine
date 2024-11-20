use std::ops::{Add, Mul, Div, Sub, Neg, Deref};
use super::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix3 {
    m: [[f32; 3]; 3]
}

impl Matrix3 {
    // Matriz identidade
    pub const I: Self = Self {
        m: [[1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]]
    };

    #[inline(always)]
    #[must_use]
    pub fn new(m: [[f32; 3]; 3]) -> Self { Self { m } }
}

// Matrix + Matrix
impl Add<Self> for Matrix3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            m: [[self.m[0][0] + rhs.m[0][0], self.m[0][1] + rhs.m[0][1], self.m[0][2] + rhs.m[0][2]],
                [self.m[1][0] + rhs.m[1][0], self.m[1][1] + rhs.m[1][1], self.m[1][2] + rhs.m[1][2]],
                [self.m[2][0] + rhs.m[2][0], self.m[2][1] + rhs.m[2][1], self.m[2][2] + rhs.m[2][2]]]
        }
    }
}

// Matrix - Matrix
impl Sub<Self> for Matrix3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            m: [[self.m[0][0] - rhs.m[0][0], self.m[0][1] - rhs.m[0][1], self.m[0][2] - rhs.m[0][2]],
                [self.m[1][0] - rhs.m[1][0], self.m[1][1] - rhs.m[1][1], self.m[1][2] - rhs.m[1][2]],
                [self.m[2][0] - rhs.m[2][0], self.m[2][1] - rhs.m[2][1], self.m[2][2] - rhs.m[2][2]]]
        }
    }
}

// Matrix multiplication
impl Mul<Self> for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3 {
            m: [
                [
                    self.m[0][0] * rhs.m[0][0] + self.m[0][1] * rhs.m[1][0] + self.m[0][2] * rhs.m[2][0],
                    self.m[0][0] * rhs.m[0][1] + self.m[0][1] * rhs.m[1][1] + self.m[0][2] * rhs.m[2][1],
                    self.m[0][0] * rhs.m[0][2] + self.m[0][1] * rhs.m[1][2] + self.m[0][2] * rhs.m[2][2]
                ],
                [
                    self.m[1][0] * rhs.m[0][0] + self.m[1][1] * rhs.m[1][0] + self.m[1][2] * rhs.m[2][0],
                    self.m[1][0] * rhs.m[0][1] + self.m[1][1] * rhs.m[1][1] + self.m[1][2] * rhs.m[2][1],
                    self.m[1][0] * rhs.m[0][2] + self.m[1][1] * rhs.m[1][2] + self.m[1][2] * rhs.m[2][2]
                ],
                [
                    self.m[2][0] * rhs.m[0][0] + self.m[2][1] * rhs.m[1][0] + self.m[2][2] * rhs.m[2][0],
                    self.m[2][0] * rhs.m[0][1] + self.m[2][1] * rhs.m[1][1] + self.m[2][2] * rhs.m[2][1],
                    self.m[2][0] * rhs.m[0][2] + self.m[2][1] * rhs.m[1][2] + self.m[2][2] * rhs.m[2][2]
                ]
            ]
        }
    }
}

// Matrix3 * Vec3
impl Mul<Vec3> for Matrix3 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            rhs.x*self.m[0][0] + rhs.y*self.m[0][1] + rhs.z*self.m[0][2],
            rhs.x*self.m[1][0] + rhs.y*self.m[1][1] + rhs.z*self.m[1][2],
            rhs.x*self.m[2][0] + rhs.y*self.m[2][1] + rhs.z*self.m[2][2]
        )
    }
}

// Matrix * n
impl Mul<f32> for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            m: [[self.m[0][0] * rhs, self.m[0][1] * rhs, self.m[0][2] * rhs],
                [self.m[1][0] * rhs, self.m[1][1] * rhs, self.m[1][2] * rhs],
                [self.m[2][0] * rhs, self.m[2][1] * rhs, self.m[2][2] * rhs]]
        }
    }
}

// Matrix / n
impl Div<f32> for Matrix3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            m: [[self.m[0][0] / rhs, self.m[0][1] / rhs, self.m[0][2] / rhs],
                [self.m[1][0] / rhs, self.m[1][1] / rhs, self.m[1][2] / rhs],
                [self.m[2][0] / rhs, self.m[2][1] / rhs, self.m[2][2] / rhs]]
        }
    }
}

// -Matrix
impl Neg for Matrix3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            m: [[-self.m[0][0], -self.m[0][1], -self.m[0][2]],
                [-self.m[1][0], -self.m[1][1], -self.m[1][2]],
                [-self.m[2][0], -self.m[2][1], -self.m[2][2]]]
        }
    }
}

// Matrix[l]
impl Deref for Matrix3 {
    type Target = [[f32; 3]; 3];

    fn deref(&self) -> &Self::Target {
        &self.m
    }
}