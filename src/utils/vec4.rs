use std::ops::{Add, Mul, Div, Sub, Neg, AddAssign};
use super::{Matrix4, Vec3}; // Assuming you have a Matrix4 class similar to Matrix3

/// Vetor 4D x,y,z,w (f64)
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

unsafe impl Send for Vec4 {}

impl Vec4 {
    pub const NULL: Vec4 = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const UP: Vec4 = Self { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };
    pub const DOWN: Vec4 = Self { x: 0.0, y: -1.0, z: 0.0, w: 0.0 };
    pub const LEFT: Vec4 = Self { x: -1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const RIGHT: Vec4 = Self { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const BACK: Vec4 = Self { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    pub const FORWARD: Vec4 = Self { x: 0.0, y: 0.0, z: -1.0, w: 0.0 };

    #[inline(always)]
    #[must_use]
    /// Constructor
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self { Self { x, y, z, w } }

    pub fn from_vec3(v: Vec3) -> Self { Self { x: v.x, y: v.y, z: v.z, w: 1.0 } }

    #[inline]
    #[must_use]
    /// Constructor x=y=z=w
    pub fn all(a: f64) -> Self { Self { x: a, y: a, z: a, w: a } }

    #[inline]
    #[must_use]
    /// Produto escalar entre dois vetores
    pub fn dot(&self, rhs: Self) -> f64 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w }

    #[inline]
    #[must_use]
    /// Retorna uma cópia do vetor normalizado
    pub fn normalize(self) -> Self { self / self.length() }

    #[inline]
    #[must_use]
    /// Retorna o tamanho do vetor ao quadrado
    pub fn length_squared(&self) -> f64 { self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w }

    #[inline]
    #[must_use]
    /// Retorna o tamanho do vetor
    pub fn length(&self) -> f64 { self.length_squared().sqrt() }

    #[inline]
    #[must_use]
    /// Retorna o vetor com cada elemento restrito entre um intervalo [min, max]
    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
            w: self.w.clamp(min, max),
        }
    }

    #[inline]
    #[must_use]
    /// Converte o vetor pra valores rgb entre 0 e 1
    pub fn rgb_normal(&self) -> Self {
        Self {
            x: self.x / 255.0,
            y: self.y / 255.0,
            z: self.z / 255.0,
            w: self.w,
        }
    }

    #[inline]
    #[must_use]
    /// Converte o vetor pra valores rgb entre 0 e 255
    pub fn rgb_255(&self) -> Self {
        Self {
            x: self.x * 255.0,
            y: self.y * 255.0,
            z: self.z * 255.0,
            w: self.w,
        }
    }
    
    #[inline]
    #[must_use]
    pub fn by_transpost(&self, rhs: Self) -> Matrix4 {
        Matrix4::new(
            [[self.x * rhs.x, self.x * rhs.y, self.x * rhs.z, self.x * rhs.w],
            [self.y * rhs.x, self.y * rhs.y, self.y * rhs.z, self.y * rhs.w],
            [self.z * rhs.x, self.z * rhs.y, self.z * rhs.z, self.z * rhs.w],
            [self.w * rhs.x, self.w * rhs.y, self.w * rhs.z, self.w * rhs.w]]
        )
    }

    #[inline]
    #[must_use]
    pub fn projection_matrix(&self) -> Matrix4 {
        Matrix4::new(
            [[self.x * self.x, self.x * self.y, self.x * self.z, self.x * self.w],
             [self.y * self.x, self.y * self.y, self.y * self.z, self.y * self.w],
             [self.z * self.x, self.z * self.y, self.z * self.z, self.z * self.w],
             [self.w * self.x, self.w * self.y, self.w * self.z, self.w * self.w]]
        )
    }

    pub fn orth_projection_matrix(&self) -> Matrix4 {
        Matrix4::I - Matrix4::new(
            [[self.x * self.x, self.x * self.y, self.x * self.z, self.x * self.w],
             [self.y * self.x, self.y * self.y, self.y * self.z, self.y * self.w],
             [self.z * self.x, self.z * self.y, self.z * self.z, self.z * self.w],
             [self.w * self.x, self.w * self.y, self.w * self.z, self.w * self.w]]
        )
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Vec4) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign<Vec4> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Vec4) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Vec4> for f64 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    /// Element-wise multiplication of two vectors.
    fn mul(self, rhs: Vec4) -> Self {
        Vec4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl Div<f64> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    /// Element-wise division of two vectors.
    fn div(self, rhs: Vec4) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}