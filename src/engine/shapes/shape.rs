use super::Material;
use super::Sphere;
use super::Plane;
use super::super::Ray;
use crate::utils::Vec3;

// TODO: struct instead of enum?
#[derive(Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane)
}

impl Shape {
    #[inline]
    #[must_use]
    pub fn normal(&self, p: &Vec3) -> Vec3 {
        match self {
            Self::Sphere(sphere) => { sphere.normal(p) }
            Self::Plane(plane) => { plane.normal }
        }
    }

    #[inline]
    #[must_use]
    pub fn material(&self) -> &Material {
        match self {
            Self::Sphere(sphere) => { &sphere.material }
            Self::Plane(plane) => { &plane.material }
        }
    }

    #[inline]
    #[must_use]
    pub fn intersects(&self, r: &Ray) -> f32 {
        match self {
            Self::Sphere(sphere) => { sphere.intersects(r) }
            Self::Plane(plane) => { plane.intersects(r) }
        }
    }
}