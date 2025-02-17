#![allow(dead_code)]
use super::shapes::Shape;
use super::Light;
use crate::utils::Vec3;

// #[derive(Clone, PartialEq)]
/// Armazena objetos e luzes com uma luz ambiente pré-definida
pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
    pub ambient_light: Vec3
}

impl Scene {
    #[inline]
    #[must_use]
    /// Cria uma nova cena
    pub fn new(shapes: Vec<Box<dyn Shape>>, lights: Vec<Light>, ambient_light: Vec3) -> Scene {
        Scene { shapes, lights, ambient_light }
    }

    #[inline]
    #[must_use]
    /// Cria uma nova cena vazia com luz ambiente definida
    pub fn new_empty(ambient_light: Vec3) -> Scene {
        Scene { shapes: Vec::new(), lights: Vec::new(), ambient_light }
    }

    #[inline]
    /// Adiciona um objeto na cena
    pub fn add_shape(&mut self, s: Box<dyn Shape>) {
        self.shapes.push(s);
    }

    #[inline]
    /// Adiciona uma luz na cena
    pub fn add_light(&mut self, l: Light) {
        self.lights.push(l);
    }
}