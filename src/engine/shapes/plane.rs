#![allow(dead_code)]
use super::Material;
use super::Shape;
use super::super::Ray;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Plano baseado num ponto `pc` com vetor normal `normal`, de material `material`.
pub struct Plane {
    pub pc: Vec3, // ponto conhecido do plano
    pub normal: Vec3,
    pub material: Material,
}


impl Plane {
    #[inline]
    #[must_use]
    /// Cria um novo plano baseado num ponto `pc` com vetor normal `normal`, de material `material`. \
    /// (Encapsulado em um enum Shape)
    pub fn new(pc: Vec3, normal: Vec3, material: Material ) -> Box<dyn Shape> {
        Box::new( Self { pc, normal: normal.normalize(), material } )
    }
}

impl Shape for Plane {
    #[inline]
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um plano e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    fn intersects(&self, r: &Ray) -> (f64, Vec3) {
        // Fórmula: n * (p - pc) = 0
        // n * (R(t) - pc) = 0
        // t = - n.dot(r.origin - pc) / n.dot(r.dr)
        // onde pc é o ponto "central" do plano e n é o vetor normal do plano
        // se n.dot(r.dr) é 0, os raio é paralelo ao plano, retorna falso (-INFINITO)
        // se não, retorna o resultado da fórmula.
        let top = self.normal.dot(r.origin - self.pc);
        let bottom = self.normal.dot(r.dr);
        if bottom == 0.0 { return (f64::NEG_INFINITY, Vec3::NULL) }
        (-top/bottom, self.normal * -self.normal.dot(r.dr).signum())
    }

    fn material(&self) -> &Material { &self.material }
}