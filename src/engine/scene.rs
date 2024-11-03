use super::shapes::Shape;
use super::Light;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Armazena objetos e luzes com uma luz ambiente pré-definida
pub struct Scene {
    pub shapes: Vec<Shape>,
    pub lights: Vec<Light>,
    pub ambient_light: Vec3
}

impl Scene {
    #[inline]
    #[must_use]
    /// Cria uma nova cena com luz ambiente definida
    pub fn new(ambient_light: Vec3) -> Scene {
        Scene { shapes: Vec::new(), lights: Vec::new(), ambient_light }
    }

    #[inline]
    /// Adiciona um objeto na cena
    pub fn add_shape(&mut self, s: Shape) {
        self.shapes.push(s);
    }

    #[inline]
    /// Adiciona uma luz na centa
    pub fn add_light(&mut self, l: Light) {
        self.lights.push(l);
    }
}