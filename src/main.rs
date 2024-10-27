mod camera;
mod sphere;
mod ray;
mod vec;

use camera::Camera;
use sphere::Sphere;
use vec::Vec3;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::time::{Duration, Instant};
use std::fs;

// fn rgb_to_normalized(r:u8, g:u8, b:u8) -> (f32, f32, f32) {
//     ((r as f32)/255.0, (g as f32)/255.0, (b as f32)/255.0)
// }

// fn normalized_to_rgb(r:f32, g:f32, b:f32) -> (u8, u8, u8) {
//     ((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8)
// }

fn save_canvas_as_ppm (canvas: &Canvas<Window>) -> Result<(), Box<dyn std::error::Error>> {
    let (w, h) = canvas.output_size()?;
    let pixels: Vec<u8> = canvas.read_pixels(Rect::new(0,0,w,h), PixelFormatEnum::RGB24)?;
    
    let mut output = String::new(); // string que guarda o output
    output += &format!("P3\n{w} {h}\n255\n"); // "header" do ppm

    for i in (0..pixels.len()).step_by(3) { // adiciona cada trio de pixels
        output += &format!("{} {} {}\n", pixels[i], pixels[i+1], pixels[i+2])
    }
    fs::write("output.ppm", output)?; // salva o arquivo.ppm

    Ok(())
}

fn main() {
    let p0 = Vec3::new(0.0, 0.0, 0.0); // posição do observador
    
    let aspect_ratio: f32 = 16.0/9.0; // aspect ratio que eu quero na imagem (16:9)
    let scale: f32 = 3.0; // cada quadrado na "câmera" vale por quantos pixels na janela do computador?

    let image_width: u32 = 320;
    let image_height: u32 = ((image_width as f32)/aspect_ratio) as u32; // imagem de 320x180 (em 16:9)
    
    let viewport_width: f32 = 3.2;
    let viewport_height: f32 = viewport_width / aspect_ratio; // janela de 3.2m * 1.8m (em 16:9)
    let viewport_distance: f32 = 1.0; // janela a 1m de distância do observador
    
    let sphere_radius = 1.0; // 1m de raio
    let sphere_center = Vec3::new(p0.x, p0.y, p0.z-(viewport_distance + sphere_radius)); // centro da esfera
    let sphere_color = Color::RGB(255, 0, 0); // cor da esfera
    
    let bg_color = Color::RGB(100, 100, 100); // cor do background
    
    let camera: Camera = Camera::new(
        p0,
        image_height, image_width,
        viewport_height, viewport_width,
        viewport_distance,
        bg_color
    );

    let mut sphere = Sphere::new(
        sphere_center, // centro da esfera
        sphere_radius, // raio da esfera
        sphere_color
    );


    // Main render loop
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap(); //
    let mut event_pump = sdl_context.event_pump().unwrap(); // cuida dos eventos como teclado mouse etc.
    let window = video_subsystem // a janela em si
        .window("CG1 - engine", ((image_width as f32)*scale) as u32, ((image_height as f32)*scale) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap(); // o canvas que a gente vai usar pra desenhar
    
    // a janela no computador vai ter um tamanho maior,
    // mas o canvas ainda é do tamanho da "grade" na câmera.
    canvas.set_logical_size(image_width, image_height).unwrap();
    camera.draw_sphere_to_canvas(&mut canvas, &sphere);
    save_canvas_as_ppm(&canvas).unwrap();
    canvas.present();
    
    // main loop do programa
    let mut frame_count = 0;
    let mut last_time = Instant::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // esc pra sair do programa
                Event::Quit{ .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                // muda a posição da bola em 10cm pra cada lado pelas setas do teclado
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => { sphere.center.x += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => { sphere.center.x -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => { sphere.center.y += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => { sphere.center.y -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { sphere.center.z -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { sphere.center.z += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::SPACE), .. } => {
                    camera.draw_sphere_to_canvas(&mut canvas, &sphere);
                    save_canvas_as_ppm(&canvas).unwrap();
                }
                _ => {}
            }
        }

        camera.draw_sphere_to_canvas(&mut canvas, &sphere); // desenha a esfera na tela ;)
        canvas.present(); // atualiza a janela com as últimas atualizações do canvas
        
        frame_count += 1;
        if last_time.elapsed() >= Duration::new(1, 0) {
            println!("FPS: {frame_count}");
            frame_count = 0;
            last_time = Instant::now();
        }
    }

}