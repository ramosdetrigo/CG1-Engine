#![allow(dead_code)]
mod vec3;
mod matrix3;
mod vec4;
mod matrix4;
pub mod transform;
use sdl2::surface::Surface;

pub use vec3::Vec3;
pub use vec4::Vec4;
pub use matrix3::Matrix3;
pub use matrix4::Matrix4;

// salva o canvas como uma imagem .ppm
pub fn save_surface_as_ppm(surface: &Surface, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (w, h) = (surface.width(), surface.height());
    let pixels: Vec<u8> = surface.without_lock().unwrap().to_vec();
    
    let mut output = String::new(); 
    output += &format!("P3\n{} {}\n255\n", w, h); 

    for i in (0..pixels.len()).step_by(4) { 
        output += &format!("{} {} {}\n", pixels[i+2], pixels[i+1], pixels[i]);
    }
    std::fs::write(file_name, output)?; 

    Ok(())
}
