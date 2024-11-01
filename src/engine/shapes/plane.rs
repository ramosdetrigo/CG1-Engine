use std::f32::INFINITY;

use super::Material;
use super::super::Ray;
use crate::utils::Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct Plane {
    pub p0: Vec3,
    pub normal: Vec3,
    pub material: Material,
}


impl Plane {
    #[inline]
    #[must_use]
    pub fn new(p0: Vec3, normal: Vec3, material: Material ) -> Self {
        Self { p0, normal, material }
    }

    pub fn intersects(&self, r: &Ray) -> (bool, f32, f32) {
        let top = self.normal.dot(r.origin - self.p0);
        let bottom = self.normal.dot(r.dr);
        if bottom == 0.0 { return ( false, -1.0, -INFINITY )}
        ( true, - top/bottom, -INFINITY )
    }
}