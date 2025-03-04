#![allow(dead_code)]
use super::Material;
use super::Shape;
use super::super::Ray;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Esfera de centro `center`, raio `radius`, e material `material`.
pub struct Sphere {
    pub center: Vec3, // Ponto x,y,z do centro da esfera
    pub radius: f64, // Raio da esfera
    pub material: Material, // Cor da esfera
}

impl Sphere {
    #[inline]
    #[must_use]
    /// Cria uma nova esfera de centro `center`, raio `radius`, e material `material`. \
    /// (Encapsulada em um enum Shape)
    pub fn new(center: Vec3, radius: f64, material: Material) -> Box<dyn Shape> {
        Box::new( Self { center, radius, material, } )
    }
}

impl Shape for Sphere {
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre uma esfera e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    fn get_intersection(&self, r: &Ray) -> Option<(f64, Vec3, Material)> {
        // Se existe um t real tal que R(t) pertence à borda da esfera, houve colisão.
        // Resolvendo a equação da esfera obtemos uma equação quadrática,
        // então só precisamos saber se o delta é positivo.
        // (C - R(t)) * (C - R(t)) = r²
        // d*d * t +  -2d*(C - p0) + (C - p0) * (C - p0) - r² = 0
        // v = (C - p0)
        // a = d*d
        // b = -2d*v
        // c = v*v - r²
        // delta = b² - 4ac
        let v: Vec3 = self.center - r.origin;
        let a: f64 = r.dr.length_squared();
        let b: f64 = r.dr.dot(v); // TODO: Explicar otimização
        let c: f64 = v.length_squared() - self.radius*self.radius;
        let delta: f64 = b*b - a*c;
        
        // se o delta é positivo, houve colisão
        if delta >= 0.0 {
            [(b + delta.sqrt()) / a, (b - delta.sqrt()) / a].into_iter()
                .filter(|t| *t > 0.0) // filtra os T's positivos
                .min_by(|t1, t2| t1.partial_cmp(t2).unwrap() ) // pega o menor deles
                .map(|t| (t, (r.at(t) - self.center).normalize(), self.material)) // retorna a interseção mais próxima, ou None se não há t positivo
        } else {
            None
        }
    }

    fn translate(&mut self, translation_vector: Vec3) {
        self.center += translation_vector;
    }

    #[inline]
    #[must_use]
    fn material(&self) -> &Material { &self.material }
}