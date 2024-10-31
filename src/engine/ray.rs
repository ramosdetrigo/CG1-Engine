// #![allow(dead_code)]
use crate::utils::vec::Vec3;

pub struct Ray {
    pub origin: Vec3, // Origem do raio
    pub dir: Vec3, // Direção do raio
}

impl Ray {
    #[inline]
    #[must_use]
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin,
            dir: dir.normalize() // direção do raio (normalizado, assim t = distância entre origin e R(t))
        }
    }
    
    // função R(t) = p0 + t*d
    // retorna o ponto P em R(t)
    #[inline]
    #[must_use]
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + t*self.dir
    }
}   