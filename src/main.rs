mod engine;
mod utils;

use engine::camera::Camera;
use engine::light_source::LightSource;
use engine::sphere::Sphere;
use utils::vec::Vec3;
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

// salva o canvas como uma imagem .ppm
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

    // imagem de 320x180 (em 16:9) (isso é o número de colunas e linhas na grade)
    let image_width: u32 = 320;
    let image_height: u32 = ((image_width as f32)/aspect_ratio) as u32;
    // janela de 3.2m * 1.8m (em 16:9)
    let viewport_width: f32 = 3.2;
    let viewport_height: f32 = viewport_width/aspect_ratio;
    let viewport_distance: f32 = 1.0; // janela a 1m de distância do observador
    
    let sphere_radius = 1.0; // 1m de raio
    let sphere_center = Vec3::new(p0.x, p0.y, p0.z - (viewport_distance + sphere_radius)); // centro da esfera (z negativo)
    let sphere_color = Color::RGB(100, 100, 100); // cor da esfera
    
    let light_pos = Vec3::new(-1.5, 1.5, -1.5);
    let light_color = Color::RGB(255, 255, 255);
    let light_intensity = 1.0;
    
    let bg_color = Color::RGB(127, 200, 255); // cor do background
    
    let camera: Camera = Camera::new(
        p0, // a posição do observador (0,0,0)
        image_width, image_height, // número de colunas e linhas na grade (basicamente a resolução)
        viewport_width, viewport_height, // tamanho da janela (em metros)
        viewport_distance, // distância da janela até o observador (em metros)
        bg_color // cor do background
    );

    let mut sphere = Sphere::new(
        sphere_center,
        sphere_radius,
        sphere_color
    );

    let mut light = LightSource::new(
        light_pos,
        light_color,
        light_intensity
    );


    // Inicializando SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap(); // cuida dos eventos como teclado mouse etc.
    let window = video_subsystem // a janela do computador em si
        .window("CG1 - engine", ((image_width as f32)*scale) as u32, ((image_height as f32)*scale) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap(); // o canvas que a gente vai usar pra desenhar
    canvas.set_logical_size(image_width, image_height).unwrap(); // pra fazer upscaling do canvas

    camera.draw_sphere(&mut canvas, &sphere); // desenha a esfera na tela ;)
    save_canvas_as_ppm(&canvas).unwrap(); // salva o que foi desenhado no canvas como uma imagem .ppm
    canvas.present(); // apresenta o canvas na tela do computador (isso também limpa o canvas)
    
    // main loop do programa
    let mut frame_count = 0; // contador de FPS no terminal
    let mut last_time = Instant::now();
    'running: loop {
        // eventos de teclado mouse etc
        for event in event_pump.poll_iter() {
            match event {
                // esc pra sair do programa
                Event::Quit{ .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                // muda a posição da bola em 10cm pra cada lado pelas setas do teclado
                // setas = eixos x,y // W,S = eixo z
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => { sphere.center.x += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => { sphere.center.x -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => { sphere.center.y += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => { sphere.center.y -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { sphere.center.z -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { sphere.center.z += 0.1; }
                // espaço pra salvar a imagem atual do canvas como .ppm
                Event::KeyDown { keycode: Some(Keycode::SPACE), .. } => {
                    camera.draw_sphere(&mut canvas, &sphere);
                    save_canvas_as_ppm(&canvas).unwrap();
                }
                _ => {}
            }
        }

        camera.draw_sphere(&mut canvas, &sphere);
        canvas.present(); // joga o que foi desenhado no canvas na janela do computador (isso também limpa o canvas)
        
        frame_count += 1;
        if last_time.elapsed() >= Duration::new(1, 0) {
            println!("FPS: {frame_count}"); // printa o número de frames desenhados no último segundo (FPS)
            frame_count = 0;
            last_time = Instant::now();
        }
    }
}