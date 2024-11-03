use super::Material;
use super::Sphere;
use super::Plane;
use super::super::Ray;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Enum Shape que "encapsula" objetos diferentes (esfera, plano)
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane)
}

impl Shape {
    #[inline]
    #[must_use]
    /// Retorna o vetor normal entre o objeto e um ponto P
    pub fn normal(&self, p: Vec3) -> Vec3 {
        match self {
            Self::Sphere(sphere) => sphere.normal(p),
            Self::Plane(plane) => plane.normal,
        }
    }

    #[inline]
    #[must_use]
    /// Retorna o material do objeto
    pub fn material(&self) -> &Material {
        match self {
            Self::Sphere(sphere) => &sphere.material,
            Self::Plane(plane) => &plane.material,
        }
    }

    #[inline]
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um objeto e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    pub fn intersects(&self, r: &Ray) -> f32 {
        match self {
            Self::Sphere(sphere) => sphere.intersects(r),
            Self::Plane(plane) => plane.intersects(r),
        }
    }
}