#![allow(dead_code)]
use super::shapes::shape::Shape;
use super::light::Light;
use crate::utils::vec::Vec3;

// #[derive(Clone, PartialEq, Debug)]
pub struct Scene {
    pub shapes: Vec<Shape>,
    pub light: Light,
    pub ambient_light: Vec3
}

impl Scene {
    pub fn new(light: Light, ambient_light: Vec3) -> Scene {
        let shapes: Vec<Shape> = Vec::new();
        Scene { shapes, light, ambient_light }
    }

    pub fn add_shape(&mut self, s: Shape) {
        self.shapes.push(s);
    }
}