#![allow(dead_code)]
use crate::engine::Ray;
use crate::utils::{Matrix3, Vec3};
use super::{Material, Shape};

#[derive(Clone, PartialEq)]
pub struct Cilinder {
    pub r: f64, pub h: f64,
    pub cb: Vec3, pub ct: Vec3,
    pub dc: Vec3,
    pub material: Material,
    pub has_base: bool,
    pub has_tampa: bool
}

impl Cilinder {
    #[inline]
    #[must_use]
    pub fn new(r: f64, h: f64, cb: Vec3, mut dc: Vec3, material: Material, has_base: bool, has_tampa: bool) -> Box<dyn Shape> {
        dc = dc.normalize();
        Box::new(Cilinder {r, h, cb, dc, ct:cb + h*dc, material, has_base, has_tampa})
    }
}

impl Shape for Cilinder {
    fn material(&self) -> &Material {
        &self.material
    }

    fn translate(&mut self, translation_vector: Vec3) {
        self.cb += translation_vector;
        self.ct += translation_vector;
    }

    #[must_use]
    fn intersects(&self, r: &Ray) -> (f64, Vec3) {
        let mut t = f64::INFINITY;
        let mut n = Vec3::NULL;

        // Check superfície cilíndrica
        let q = self.dc.projection_matrix();
        let m = Matrix3::I - q;
        let s = r.origin - self.cb;
        
        let mdr = m*r.dr;
        let ms = m*s;

        let a = mdr.length_squared();
        let b = 2.0 * mdr.dot(ms);
        let c = ms.length_squared() - self.r*self.r;
        let delta = b*b - 4.0*a*c;

        if delta >= 0.0 {
            let t1 = (-b + delta.sqrt()) / (2.0*a);
            let t2 = (-b - delta.sqrt()) / (2.0*a);

            if t1 > 0.0 && t1 < t {
                let cbp = r.at(t1) - self.cb;
                let cbe = q*cbp;

                if cbe.dot(self.dc) > 0.0 && cbe.length() < self.h {
                    t = t1;
                    n = (m*cbp).normalize();
                }
            }

            if t2 > 0.0 && t2 < t {
                let cbp = r.at(t2) - self.cb;
                let cbe = q*cbp;

                if cbe.dot(self.dc) > 0.0 && cbe.length() < self.h {
                    t = t2;
                    n = (m*cbp).normalize();
                }
            }
        }
        
        // Check plano do topo do cilindro
        if self.has_tampa {
            let bottom = r.dr.dot(self.dc);
            if bottom != 0.0 {
            let t_tampa = -(r.origin - self.ct).dot(self.dc) / bottom;
            
            if t_tampa >= 0.0
            && t_tampa < t
            && (r.at(t_tampa) - self.ct).length() <= self.r {
                t = t_tampa;
                n = self.dc;
            }
        }
        }

        // Check plano da base do cilindro
        if self.has_base {
            let bottom = r.dr.dot(-self.dc);
            if bottom != 0.0 {
                let t_base = -(r.origin - self.cb).dot(-self.dc) / bottom;
                
                if t_base >= 0.0
                && t_base < t
                && (r.at(t_base) - self.cb).length() <= self.r {
                    t = t_base;
                    n = self.dc;
                }
            }
        }
        
        if t == f64::INFINITY { t = -t }
        (t, n * -n.dot(r.dr).signum())
    }
}