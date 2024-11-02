use std::ops::{Add, Mul, Div, Sub, Neg, AddAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
/// Vetor x,y,z (f32)
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

unsafe impl Send for Vec3 {}

impl Vec3 {
    pub const NULL: Vec3 = Self { x: 0.0, y: 0.0, z: 0.0 };

    #[inline]
    #[must_use]
    /// Constructor
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self {x, y, z} }

    #[inline]
    #[must_use]
    /// Produto escalar entre dois vetores
    pub fn dot(&self, rhs: Self) -> f32 { self.x*rhs.x + self.y*rhs.y + self.z*rhs.z }

    #[inline]
    #[must_use]
    /// Retorna uma cópia do vetor normalizado
    pub fn normalize(self) -> Self { self / self.length() }

    #[inline]
    #[must_use]
    pub fn length_squared(&self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z }

    #[inline]
    #[must_use]
    /// Retorna o tamanho do vetor
    pub fn length(&self) -> f32 { (self.x*self.x + self.y*self.y + self.z*self.z).sqrt() }

    #[inline]
    #[must_use]
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max)
        }
    }

    #[inline]
    #[must_use]
    pub fn rgb_normal(&self) -> Self {
        Self {
            x: self.x / 255.0,
            y: self.y / 255.0,
            z: self.z / 255.0
        }
    }

    #[inline]
    #[must_use]
    pub fn rgb_255(&self) -> Self {
        Self {
            x: self.x * 255.0,
            y: self.y * 255.0,
            z: self.z * 255.0
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Vec3) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) { self.x += rhs.x; self.y += rhs.y; self.z += rhs.z; }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Vec3) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}