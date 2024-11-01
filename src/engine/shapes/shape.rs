use super::Material;
use super::Sphere;
use super::Plane;
use super::super::Ray;
use crate::utils::Vec3;

// TODO: struct instead of enum?
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane)
}

impl Shape {
    pub fn normal(&self, p: &Vec3) -> Vec3 {
        match self {
            Shape::Sphere(sphere) => { sphere.normal(p) }
            Shape::Plane(plane) => { plane.normal }
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(sphere) => { &sphere.material }
            Shape::Plane(plane) => { &plane.material }
        }
    }

    pub fn intersects(&self, r: &Ray) -> (bool, f32, f32) {
        match self {
            Shape::Sphere(sphere) => { sphere.intersects(r) }
            Shape::Plane(plane) => { plane.intersects(r) }
        }
    }
}