#![allow(dead_code)]
use crate::utils::vec::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LightSource {
    pub pos: Vec3,
    pub color: Vec3,
    pub intensity: f32
}

impl LightSource {
    pub fn new(pos: Vec3, color: Vec3, intensity: f32) -> LightSource {
        LightSource { pos, color, intensity }
    }
}