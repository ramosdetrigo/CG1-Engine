use super::Material;
use super::Shape;
use super::super::Ray;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Esfera de centro `center`, raio `radius`, e material `material`.
pub struct Sphere {
    pub center: Vec3, // Ponto x,y,z do centro da esfera
    pub radius: f32, // Raio da esfera
    pub material: Material, // Cor da esfera
}

impl Sphere {
    #[inline]
    #[must_use]
    /// Cria uma nova esfera de centro `center`, raio `radius`, e material `material`. \
    /// (Encapsulada em um enum Shape)
    pub fn new(center: Vec3, radius: f32, material: Material) -> Shape {
        Shape::Sphere( Self { center, radius, material, })
    }

    #[inline]
    #[must_use]
    /// Retorna o vetor normal entre o centro da esfera e um ponto `p`
    pub fn normal(&self, p: Vec3) -> Vec3 {
        (p - self.center).normalize()
    }

    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre uma esfera e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    pub fn intersects(&self, r: &Ray) -> f32 {
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
        let a: f32 = r.dr.length_squared();
        let b: f32 = r.dr.dot(v); // TODO: Explicar otimização
        let c: f32 = v.length_squared() - self.radius*self.radius;
        let delta: f32 = b*b - a*c;
        
        // se o delta é positivo, houve colisão
        if delta >= 0.0 {
            let t1 = (b + delta.sqrt()) / a;
            let t2 = (b - delta.sqrt()) / a;
            if t2 < 0.0 || t1 < t2 {t1} else {t2} // mínimo positivo
        } else {
            f32::NEG_INFINITY
        }
    }
}