#![allow(dead_code)]
use crate::utils::vec::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Light {
    pub pos: Vec3,
    pub color: Vec3,
    pub intensity: f32
}

impl Light {
    pub fn new(pos: Vec3, color: Vec3, intensity: f32) -> Light {
        Light { pos, color, intensity }
    }
}