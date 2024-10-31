#![allow(dead_code)]
use super::shapes::sphere::Sphere;
use super::shapes::plane::Plane;
use super::light::Light;
use crate::utils::vec::Vec3;

// #[derive(Clone, PartialEq, Debug)]
pub struct Scene {
    pub sphere: Sphere,
    pub plane: Plane,
    pub light: Light,
    pub ambient_light: Vec3
}

impl Scene {
    pub fn new(sphere: Sphere, plane: Plane, light: Light, ambient_light: Vec3) -> Scene {
        Scene { sphere, plane, light, ambient_light }
    }
}