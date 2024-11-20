#![allow(dead_code)]
use super::Material;
use super::Sphere;
use super::Plane;
use super::Cilinder;
use super::Cone;
use super::super::Ray;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Enum Shape que "encapsula" objetos diferentes (esfera, plano)
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cilinder(Cilinder),
    Cone(Cone)
}

impl Shape {
    #[inline]
    #[must_use]
    /// Retorna o material do objeto
    pub fn material(&self) -> &Material {
        match self {
            Self::Sphere(sphere) => &sphere.material,
            Self::Plane(plane) => &plane.material,
            Self::Cilinder(cilinder) => &cilinder.material,
            Self::Cone(cone) => &cone.material,
        }
    }

    // TODO: Normal dentro da esfera//de baixo do plano não são calculadas corretamente.
    #[inline]
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um objeto e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    pub fn intersects(&self, r: &Ray) -> (f32, Vec3) {
        match self {
            Self::Sphere(sphere) => sphere.intersects(r),
            Self::Plane(plane) => plane.intersects(r),
            Self::Cilinder(cilinder) => cilinder.intersects(r),
            Self::Cone(cone) => cone.intersects(r),
        }
    }
}