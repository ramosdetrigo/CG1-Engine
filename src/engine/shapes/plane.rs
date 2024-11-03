use std::f32::NEG_INFINITY;
use super::Material;
use super::Shape;
use super::super::Ray;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Plano baseado num ponto `p0` com vetor normal `normal`, de material `material`.
pub struct Plane {
    pub p0: Vec3,
    pub normal: Vec3,
    pub material: Material,
}


impl Plane {
    #[inline]
    #[must_use]
    /// Cria um novo plano baseado num ponto `p0` com vetor normal `normal`, de material `material`. \
    /// (Encapsulado em um enum Shape)
    pub fn new(p0: Vec3, normal: Vec3, material: Material ) -> Shape {
        Shape::Plane( Self { p0, normal, material } )
    }

    #[inline]
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um plano e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    pub fn intersects(&self, r: &Ray) -> f32 {
        // Fórmula: n * (p - p0) = 0
        // n * (R(t) - p0) = 0
        // t = - n.dot(r.origin - p0) / n.dot(r.dr)
        // onde p0 é o ponto "central" do plano e n é o vetor normal do plano
        // se n.dot(r.dr) é 0, os raio é paralelo ao plano, retorna falso (-INFINITO)
        // se não, retorna o resultado da fórmula.
        let top = self.normal.dot(r.origin - self.p0);
        let bottom = self.normal.dot(r.dr);
        if bottom == 0.0 { return NEG_INFINITY }
        -top/bottom
    }
}