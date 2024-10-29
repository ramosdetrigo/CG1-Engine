#![allow(dead_code)]
use super::sphere::Sphere;
use super::light_source::LightSource;

// #[derive(Clone, PartialEq, Debug)]
pub struct Scene {
    sphere: Sphere,
    light: LightSource
}

impl Scene {
    pub fn new(sphere: Sphere, light: LightSource) -> Scene {
        Scene {
            sphere, light
        }
    }
}