// #![allow(dead_code)]
use super::sphere::Sphere;
use super::plane::Plane;
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

    #[inline]
    #[must_use]
    pub fn intersects_plane(&self, plane: &Plane) -> (bool, f32) {
        let top = plane.normal.dot(self.origin - plane.p0);
        let bottom = plane.normal.dot(self.dir);
        if bottom == 0.0 { return ( false, -1.0 )}

        ( true, - top/bottom )
    }

    pub fn intersects_sphere(&self, sphere: &Sphere) -> (bool, f32, f32) {
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
        let v: Vec3 = sphere.center - self.origin;
        let a: f32 = self.dir.dot(self.dir);
        let b: f32 = (-2.0 * self.dir).dot(v);
        let c: f32 = v.dot(v) - sphere.radius*sphere.radius;
        let delta: f32 = b*b - 4.0*a*c;
        
        // se o delta é positivo e != 0 (não apenas tangencia a esfera), houve colisão
        if delta > 0.0 {
            let t1 = (-b + delta.sqrt()) / (2.0*a);
            let t2 = (-b - delta.sqrt()) / (2.0*a);
            return (true, t1, t2);
        } else {
            return (false, -1.0, -1.0);
        }

    }
}   