#![allow(dead_code)]
pub mod vec;
use vec::Vec3;
use sdl2::pixels::Color;

pub fn vec_to_color(v: Vec3) -> Color {
    Color::RGB(v.x as u8, v.y as u8, v.z as u8)
}

pub fn color_to_vec(c: Color) -> Vec3 {
    Vec3::new(c.r as f32, c.g as f32, c.b as f32)
}