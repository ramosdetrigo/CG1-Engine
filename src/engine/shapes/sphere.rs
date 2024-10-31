use crate::utils::vec::Vec3;
// use core::borrow::Borrow;

#[derive(Clone, PartialEq, Debug)]
pub struct Sphere {
    pub center: Vec3, // Ponto x,y,z do centro da esfera
    pub radius: f32, // Raio da esfera
    pub color: Vec3, // Cor da esfera
    
    pub k_ambiente: Vec3, // Refletividade com a luz ambiente
    pub k_difuso: Vec3, // Refletividade difusa
    pub k_especular: Vec3, // Refletividade especular
    pub e: f32, // Coeficiente de espelhamento
}

impl Sphere {
    #[inline]
    #[must_use]
    pub fn new(center: Vec3, radius: f32, color: Vec3, k_ambiente: Vec3, k_difuso: Vec3, k_especular: Vec3, e: f32) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            k_ambiente,
            k_difuso,
            k_especular,
            e
        }
    }
}