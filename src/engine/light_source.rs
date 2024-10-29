#![allow(dead_code)]
use crate::utils::vec::Vec3;
use sdl2::pixels::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LightSource {
    pub pos: Vec3,
    pub color: Color,
    pub intensity: f32
}

impl LightSource {
    pub fn new(pos: Vec3, color: Color, intensity: f32) -> LightSource {
        LightSource { pos, color, intensity }
    }
}