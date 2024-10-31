use crate::utils::vec::Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct Plane {
    pub p0: Vec3,
    pub normal: Vec3,
    pub color: Vec3,

    pub k_ambiente: Vec3, // Refletividade com a luz ambiente
    pub k_difuso: Vec3, // Refletividade difusa
    pub k_especular: Vec3, // Refletividade especular
    pub e: f32, // Coeficiente de espelhamento
}


impl Plane {
    #[inline]
    #[must_use]
    pub fn new(p0: Vec3, normal: Vec3, color: Vec3, k_ambiente: Vec3, k_difuso: Vec3, k_especular: Vec3, e: f32) -> Self {
        Self {
            p0, normal, color,
            k_ambiente, k_difuso, k_especular, e
        }
    }
}