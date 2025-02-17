#![allow(dead_code)]
use super::Material;
use super::super::Ray;
use crate::utils::Vec3;

/// Enum Shape que "encapsula" objetos diferentes (esfera, plano)
pub trait Shape : Sync {
    #[must_use]
    /// Retorna o material do objeto
    fn material(&self) -> &Material;

    // TODO: Normal dentro da esfera//de baixo do plano não são calculadas corretamente.
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um objeto e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    fn intersects(&self, r: &Ray) -> (f64, Vec3);

    // fn apply_transform(&mut self, transform: Transform);

    fn translate(&mut self, translation_vector: Vec3);

    // fn scale(&mut self, scaling_vector: Vec3);

    // fn rotate(&mut self, rotation_vector: Vec3);
}