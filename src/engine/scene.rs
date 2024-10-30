#![allow(dead_code)]
use super::sphere::Sphere;
use super::plane::Plane;
use super::light_source::LightSource;
use crate::utils::vec::Vec3;

// #[derive(Clone, PartialEq, Debug)]
pub struct Scene {
    pub sphere: Sphere,
    pub plane: Plane,
    pub light: LightSource,
    pub ambient_light: Vec3
}

impl Scene {
    pub fn new(sphere: Sphere, plane: Plane, light: LightSource, ambient_light: Vec3) -> Scene {
        Scene { sphere, plane, light, ambient_light }
    }
}