#![allow(dead_code)]
use super::Material;
use super::Shape;
use super::super::Ray;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Triângulo definido por três vértices `v0`, `v1`, `v2` e material `material`.
pub struct Triangle {
    pub v0: Vec3, // Primeiro vértice
    pub v1: Vec3, // Segundo vértice
    pub v2: Vec3, // Terceiro vértice
    pub material: Material, // Material do triângulo
}

impl Triangle {
    #[inline]
    #[must_use]
    /// Cria um novo triângulo com os vértices `v0`, `v1`, `v2` e material `material`.
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Box<dyn Shape> {
        Box::new( Self { v0, v1, v2, material } )
    }
}

impl Shape for Triangle {
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um triângulo e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    fn intersects(&self, r: &Ray) -> (f64, Vec3) {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = r.dr.cross(edge2);
        let a = edge1.dot(h);

        if a > -1e-8 && a < 1e-8 {
            return (f64::NEG_INFINITY, Vec3::NULL); // O raio é paralelo ao triângulo
        }

        let f = 1.0 / a;
        let s = r.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return (f64::NEG_INFINITY, Vec3::NULL); // O ponto está fora do triângulo
        }

        let q = s.cross(edge1);
        let v = f * r.dr.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return (f64::NEG_INFINITY, Vec3::NULL); // O ponto está fora do triângulo
        }

        // Cálculo do t para encontrar o ponto de interseção
        let t = f * edge2.dot(q);
        if t > 1e-8 { // O raio intersecta o triângulo
            let n = edge1.cross(edge2).normalize(); // Normal do triângulo
            return (t, n);
        } else {
            return (f64::NEG_INFINITY, Vec3::NULL); // O triângulo está atrás do raio
        }
    }

    #[inline]
    #[must_use]
    fn material(&self) -> &Material { &self.material }
}