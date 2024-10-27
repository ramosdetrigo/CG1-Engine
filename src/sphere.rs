use crate::vec::Vec3;
use sdl2::pixels::Color;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color
}

impl Sphere {
    #[inline]
    #[must_use]
    pub fn new(center: Vec3, radius: f32, color: Color) -> Sphere {
        Sphere {
            center,
            radius,
            color
        }
    }
}