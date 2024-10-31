use super::material::Material;
use crate::utils::vec::Vec3;
use super::super::ray::Ray;

#[derive(Clone, PartialEq, Debug)]
pub struct Sphere {
    pub center: Vec3, // Ponto x,y,z do centro da esfera
    pub radius: f32, // Raio da esfera
    pub material: Material, // Cor da esfera
}

impl Sphere {
    #[inline]
    #[must_use]
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self { center, radius, material, }
    }

    pub fn normal(&self, p: &Vec3) -> Vec3 {
        (*p - self.center).normalize()
    }

    pub fn intersects(&self, r: &Ray) -> (bool, f32, f32) {
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
        let a: f32 = r.dir.dot(r.dir);
        let b: f32 = (-2.0 * r.dir).dot(v);
        let c: f32 = v.dot(v) - self.radius*self.radius;
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