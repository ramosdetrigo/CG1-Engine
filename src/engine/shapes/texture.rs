use sdl2::surface::Surface;
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
pub struct Texture {
    texture_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u32,
}

impl Texture {
    pub fn new(surface: Surface) -> Self {
        let width = surface.width();
        let height = surface.height();
        let pitch = surface.pitch();
        let bpp = pitch / width;
        let texture_data: Vec<u8> = surface.without_lock()
            .unwrap()
            .iter()
            .map(|n| *n)
            .collect();
        Self {
            width, height, pitch,
            texture_data, bpp
        }
    }

    pub fn sample(&self, u: f64, v: f64) -> Vec3 {
        let x = (u * self.width as f64).round() as u32;
        let y = (v * self.height as f64).round() as u32;

        let index = (y*self.pitch + x*self.bpp) as usize;
        let r = *self.texture_data.get(index).unwrap_or(&0);
        let g = *self.texture_data.get(index+1).unwrap_or(&0);
        let b = *self.texture_data.get(index+2).unwrap_or(&0);
        Vec3::new(
            r as f64 / 255.0,
            g as f64 / 255.0,
            b as f64 / 255.0,
        )
    }
}
