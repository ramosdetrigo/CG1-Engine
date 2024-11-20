use crate::engine::Ray;
use crate::utils::{Matrix3, Vec3};
use super::{Material, Shape};

#[derive(Clone, PartialEq)]
pub struct Cone {
    pub r: f32, pub h: f32,
    pub cb: Vec3, pub v: Vec3,
    pub dc: Vec3,
    pub material: Material
}

impl Cone {
    #[inline]
    #[must_use]
    pub fn new(r: f32, h: f32, cb: Vec3, mut dc: Vec3, material: Material) -> Shape {
        dc = dc.normalize();
        Shape::Cone( Self { r, h, cb, v: cb + dc*h, dc, material } )
    }

    #[must_use]
    pub fn intersects(&self, r: &Ray) -> (f32, Vec3) {
        let mut t = f32::INFINITY;
        let mut n = Vec3::NULL;

        // Check superfície cônica
        let q = self.dc.projection_matrix();
        let m = Matrix3::I - q;
        let s = r.origin - self.cb;

        let mdr = m*r.dr;
        let ms = m*s;
        let qdr = q*r.dr;
        let qs = q*s;
        let hdc = self.h * self.dc;
        let h2 = self.h*self.h;
        let r2 = self.r*self.r;

        let a = h2*mdr.length_squared() - r2*qdr.length_squared();
        let b = 2.0 * ( h2*mdr.dot(ms) + r2*qdr.dot(hdc - qs) );
        let c = h2*ms.length_squared() - r2*(hdc-qs).length_squared();
        let delta = b*b - 4.0*a*c;

        if delta >= 0.0 {
            let t1 = (-b + delta.sqrt()) / (2.0*a);
            let t2 = (-b - delta.sqrt()) / (2.0*a);

            if t1 > 0.0 && t1 < t {
                let cbp = r.at(t1) - self.cb;
                let cbe = q*cbp;

                if cbe.dot(self.dc) > 0.0
                && cbe.length() < self.h {
                    t = t1;
                    n = (m*cbp).normalize();
                }
            }

            if t2 > 0.0 && t2 < t {
                let cbp = r.at(t2) - self.cb;
                let cbe = q*cbp;

                if cbe.dot(self.dc) > 0.0
                && cbe.length() < self.h {
                    t = t2;
                    n = (m*cbp).normalize();
                }
            }
        }

        // Check plano da base do cone
        let bottom = r.dr.dot(-self.dc);
        if bottom != 0.0 {
            let t_base = -(r.origin - self.cb).dot(-self.dc) / bottom;
            
            if t_base >= 0.0
            && t_base < t
            && (r.at(t_base) - self.cb).length() <= self.r {
                t = t_base;
                n = self.dc * -self.dc.dot(r.dr).signum();
            }
        }

        if t == f32::INFINITY { t = -t; }
        (t, n)
    }
}