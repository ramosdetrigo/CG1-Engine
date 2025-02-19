#![allow(dead_code)]
use super::shapes::Shape;
use super::{Light, Ray};
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

    pub fn get_closest_positive_intersection(&self, ray: &Ray) -> (Option<&Box<dyn Shape>>, f64, Vec3) {
        let mut closest_shape = None;
        let mut t = f64::INFINITY;
        let mut n = Vec3::NULL;
        for shape in &self.shapes {
            let (t_candidate, n_candidate) = shape.get_intersection(&ray);
            // se o objeto colide com o raio, não está atrás do observador, e tá mais próximo que todo objeto testado até agr
            if t_candidate > 0.0 && t_candidate < t {
                closest_shape = Some(shape);
                t = t_candidate;
                n = n_candidate;
            }
        }
        if t == f64::INFINITY { (closest_shape, -t, n) }
        else { (closest_shape, t, n) }
    }
}