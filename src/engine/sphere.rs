use crate::utils::vec::Vec3;
use sdl2::pixels::Color;
// use core::borrow::Borrow;

#[derive(PartialEq, Debug)]
pub struct Sphere {
    pub center: Vec3, // Ponto x,y,z do centro da esfera
    pub radius: f32, // Raio da esfera
    pub color: Color // Cor da esfera
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