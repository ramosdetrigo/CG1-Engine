mod engine;
mod utils;

use engine::Camera;
use engine::Light;
use engine::Scene;
use engine::shapes::Material;
use engine::shapes::Sphere;
use engine::shapes::Shape;
use engine::shapes::Plane;
use utils::Vec3;
use utils::vec_to_color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use std::time::{Duration, Instant};

// salva o canvas como uma imagem .ppm
fn save_canvas_as_ppm (canvas: &Canvas<Window>) -> Result<(), Box<dyn std::error::Error>> {
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

fn main() {
    let p0 = Vec3::new(0.0, 0.0, 0.0); // posição do observador
    
    let aspect_ratio: f32 = 16.0/9.0; // aspect ratio que eu quero na imagem (16:9)
    let scale: f32 = 2.0; // cada quadrado na "câmera" vale por quantos pixels na janela do computador?

    // imagem de 320x180 (em 16:9) (isso é o número de colunas e linhas na grade)
    let image_width: u32 = 960;
    let image_height: u32 = ((image_width as f32)/aspect_ratio) as u32;
    // janela de 3.2m * 1.8m (em 16:9)
    let viewport_width: f32 = 3.2;
    let viewport_height: f32 = viewport_width/aspect_ratio;
    let viewport_distance: f32 = 1.0; // janela a 1m de distância do observador
    
    let sphere1_radius = 0.5; // 1m de raio
    let sphere1_center = Vec3::new(-1.2, p0.y, p0.z - (viewport_distance + sphere1_radius)); // centro da esfera (z negativo)
    let sphere1_material = Material::new(
        Vec3::new(0.1, 0.0, 0.0), // Ambient
        Vec3::new(0.7, 0.0, 0.0), // Diffuse
        Vec3::new(0.3, 0.3, 0.3), // Specular
        5.0, // e
    );

    let sphere2_radius = 0.5; // 1m de raio
    let sphere2_center = Vec3::new(0.0, p0.y, p0.z - (viewport_distance + sphere2_radius)); // centro da esfera (z negativo)
    let sphere2_material = Material::new(
        Vec3::new(0.0, 0.1, 0.0), // Ambient
        Vec3::new(0.0, 0.7, 0.0), // Diffuse
        Vec3::new(0.3, 0.3, 0.3), // Specular
        5.0, // e
    );

    let sphere3_radius = 0.5; // 1m de raio
    let sphere3_center = Vec3::new(1.2, p0.y, p0.z - (viewport_distance + sphere3_radius)); // centro da esfera (z negativo)
    let sphere3_material = Material::new(
        Vec3::new(0.0, 0.0, 0.1), // Ambient
        Vec3::new(0.0, 0.0, 0.7), // Diffuse
        Vec3::new(0.3, 0.3, 0.3), // Specular
        5.0, // e
    );
    
    let plane_p0 = Vec3::new(0.0, -1.8, 0.0);
    let plane_normal = Vec3::new(0.0, 1.0 ,0.0);
    let plane_material = Material::new(
        Vec3::new(0.1, 0.1, 0.1), // Ambient
        Vec3::new(0.7, 0.7, 0.7), // Diffuse
        Vec3::new(0.3, 0.3, 0.3), // Specular
        5.0, // e
    );
    
    let light1_pos = Vec3::new(-1.6, 0.8, 0.0);
    let light1_color = Vec3::new(1.0, 0.0, 0.0);
    let light1_intensity = 1.0;

    let light2_pos = Vec3::new(0.0, 0.8, 0.0);
    let light2_color = Vec3::new(0.0, 1.0, 0.0);
    let light2_intensity = 1.0;

    let light3_pos = Vec3::new(1.6, 0.8, 0.0);
    let light3_color = Vec3::new(0.0, 0.0, 1.0);
    let light3_intensity = 1.0;
    
    let ambient_light = Vec3::new(1.0, 1.0, 1.0);
    let bg_color = vec_to_color(Vec3::new(0.0,0.0,0.0).rgb_255()); // cor do background
    
    let mut camera: Camera = Camera::new(
        p0, // a posição do observador (0,0,0)
        image_width, image_height, // número de colunas e linhas na grade (basicamente a resolução)
        viewport_width, viewport_height, // tamanho da janela (em metros)
        viewport_distance, // distância da janela até o observador (em metros)
        bg_color // cor do background
    );

    let sphere1 = Sphere::new( sphere1_center, sphere1_radius, sphere1_material );
    let sphere2 = Sphere::new( sphere2_center, sphere2_radius, sphere2_material );
    let sphere3 = Sphere::new( sphere3_center, sphere3_radius, sphere3_material );
    let plane = Plane::new( plane_p0, plane_normal, plane_material );
    let light1 = Light::new( light1_pos, light1_color, light1_intensity ); 
    let light2 = Light::new( light2_pos, light2_color, light2_intensity );
    let light3 = Light::new( light3_pos, light3_color, light3_intensity );
    
    let mut scene = Scene::new(ambient_light);
    scene.add_shape(Shape::Plane(plane));
    scene.add_shape(Shape::Sphere(sphere1));
    scene.add_shape(Shape::Sphere(sphere2));
    scene.add_shape(Shape::Sphere(sphere3));
    scene.add_light(light1);
    scene.add_light(light2);
    scene.add_light(light3);

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

    camera.draw_scene(&mut canvas, &scene); // desenha a esfera na tela ;)
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
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => { scene.lights[0].pos.x += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => { scene.lights[0].pos.x -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => { scene.lights[0].pos.z -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => { scene.lights[0].pos.z += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { scene.lights[0].pos.y += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { scene.lights[0].pos.y -= 0.1; }
                // espaço pra salvar a imagem atual do canvas como .ppm
                Event::KeyDown { keycode: Some(Keycode::SPACE), .. } => {
                    camera.draw_scene(&mut canvas, &scene);
                    save_canvas_as_ppm(&canvas).unwrap();
                }
                _ => {}
            }
        }

        camera.draw_scene(&mut canvas, &scene);
        canvas.present(); // joga o que foi desenhado no canvas na janela do computador (isso também limpa o canvas)
        
        frame_count += 1;
        if last_time.elapsed() >= Duration::new(1, 0) {
            println!("FPS: {frame_count}"); // printa o número de frames desenhados no último segundo (FPS)
            frame_count = 0;
            last_time = Instant::now();
            // println!("{:?}", scene.light.pos);
        }
    }
}