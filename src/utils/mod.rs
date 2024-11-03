#![allow(dead_code)]
mod vec;
pub use vec::Vec3;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[inline]
/// Converte um vetor 3D em um objeto Color do SDL
pub fn vec_to_color(v: Vec3) -> Color {
    Color::RGB(v.x as u8, v.y as u8, v.z as u8)
}

#[inline]
/// Converte um objeto Color do SDL em um vetor 3D
pub fn color_to_vec(c: Color) -> Vec3 {
    Vec3::new(c.r as f32, c.g as f32, c.b as f32)
}

// salva o canvas como uma imagem .ppm
pub fn save_canvas_as_ppm (canvas: &Canvas<Window>) -> Result<(), Box<dyn std::error::Error>> {
    let (w, h) = canvas.output_size()?;
    let pixels: Vec<u8> = canvas.read_pixels(Rect::new(0,0,w,h), PixelFormatEnum::RGB24)?;
    
    let mut output = String::new(); // string que guarda o output
    output += &format!("P3\n{w} {h}\n255\n"); // "header" do ppm

    for i in (0..pixels.len()).step_by(3) { // adiciona cada trio de pixels
        output += &format!("{} {} {}\n", pixels[i], pixels[i+1], pixels[i+2])
    }
    std::fs::write("output.ppm", output)?; // salva o arquivo.ppm

    Ok(())
}