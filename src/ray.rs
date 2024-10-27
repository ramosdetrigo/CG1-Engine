// #![allow(dead_code)]
use crate::vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

// função R(t) = p0 + t*d
impl Ray {
    #[inline]
    #[must_use]
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin,
            dir: dir.normalized() // direção do raio (normalizado)
        }
    }

    // retorna o ponto P em R(t)
    #[inline]
    #[must_use]
    pub fn at(self: Ray, t: f32) -> Vec3 {
        self.origin + t*self.dir
    }
}   