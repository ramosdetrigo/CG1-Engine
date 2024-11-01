use super::shapes::Shape;
use super::Light;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
pub struct Scene {
    pub shapes: Vec<Shape>,
    pub light: Light,
    pub ambient_light: Vec3
}

impl Scene {
    #[inline]
    #[must_use]
    pub fn new(light: Light, ambient_light: Vec3) -> Scene {
        Scene { shapes: Vec::new(), light, ambient_light }
    }

    #[inline]
    pub fn add_shape(&mut self, s: Shape) {
        self.shapes.push(s);
    }
}