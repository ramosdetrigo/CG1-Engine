#![allow(dead_code)]
use std::any::Any;
use super::Material;
use super::super::Ray;
use crate::utils::{Vec3, Matrix4};

/// Enum Shape que "encapsula" objetos diferentes (esfera, plano)
pub trait Shape : Sync + Any {
    #[must_use]
    /// Retorna o material do objeto
    fn material(&self) -> &Material;

    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um objeto e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    fn get_intersection(&self, r: &Ray) -> Option<(f64, Vec3, Material)>;

    fn translate(&mut self, translation_vector: Vec3);

    fn transform(&mut self, matrix: &Matrix4);
    
    fn as_any(&mut self) -> &mut dyn Any;

    // fn rotate(&mut self, rotation_vector: Vec3);
}