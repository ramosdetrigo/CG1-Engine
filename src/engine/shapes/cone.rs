#![allow(dead_code)]
use crate::engine::Ray;
use crate::utils::{Matrix3, Vec3};
use super::{Material, Shape};

#[derive(Clone, PartialEq)]
pub struct Cone {
    pub r: f64, pub h: f64,
    pub cb: Vec3, pub v: Vec3,
    pub dc: Vec3,
    pub material: Material,
    pub has_base: bool
}

impl Cone {
    #[inline]
    #[must_use]
    pub fn new(r: f64, h: f64, cb: Vec3, mut dc: Vec3, material: Material, has_base: bool) -> Box<dyn Shape> {
        dc = dc.normalize();
        Box::new( Self { r, h, cb, v: cb + dc*h, dc, material, has_base } )
    }
}

impl Shape for Cone {
    fn translate(&mut self, translation_vector: Vec3) {
        self.cb += translation_vector;
        self.v += translation_vector;
    }

    #[must_use]
    fn get_intersection(&self, r: &Ray) -> Option<(f64, Vec3)> {
        let mut closest_intersection: Option<(f64, Vec3)> = None;
        let mut min_t = f64::INFINITY;

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
            
            // Encontra o menor t com uma interseção válida na superfíce cônica
            closest_intersection = (if t1 > t2 { [t2, t1] } else { [t1, t2] })
                .into_iter()
                .find_map(|t| {
                    let cbp = r.at(t) - self.cb;
                    let cbe = q*cbp;

                    if cbe.dot(self.dc) > 0.0 && cbe.length() < self.h {
                        min_t = t;
    
                        let p = r.at(t);
                        let pv = (self.v-p).normalize();
                        let m_pv = pv.orth_projection_matrix();
    
                        Some((t, (m_pv*self.dc).normalize()))
                    } else {
                        None
                    }
                });
        }

        // Check plano da base do cone
        if self.has_base {
            let bottom = r.dr.dot(-self.dc);
            if bottom != 0.0 {
                let t_base = -(r.origin - self.cb).dot(-self.dc) / bottom;
                
                if t_base >= 0.0
                && t_base < min_t
                && (r.at(t_base) - self.cb).length() <= self.r {
                    closest_intersection = Some( (t_base, self.dc) )
                }
            }
        }

        closest_intersection.map(|(t, n)| (t, n * -n.dot(r.dr).signum()) )
    }

    fn material(&self) -> &Material { &self.material }
}