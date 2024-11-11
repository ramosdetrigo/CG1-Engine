mod engine;
mod utils;

use utils::{Vec3, save_canvas_as_ppm};
use engine::{Camera, Light, Scene};
use engine::shapes::{Material, Sphere, Plane};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

fn main() {
    let p0 = Vec3::new(0.0, 0.0, 0.0); // posição do observador
    
    let aspect_ratio: f32 = 16.0/9.0; // aspect ratio que eu quero
    let scale: f32 = 1.0; // escala: cada quadrado na "janela" vale por quantos pixels na janela do computador?

    // Resolução da imagem (isso é o número de colunas e linhas na grade)
    let image_width: u32 = 1920;
    let image_height: u32 = ((image_width as f32)/aspect_ratio) as u32;
    // Tamanho da janela (em metros)
    let viewport_width: f32 = 3.2;
    let viewport_height: f32 = viewport_width/aspect_ratio;
    let viewport_distance: f32 = 1.0; // distância da janela até o observador
    
    let bg_color = Vec3::new(0.0,0.0,0.0); // cor do background
    
    let mut camera: Camera = Camera::new(
        p0, // a posição do observador
        image_width, image_height, // número de colunas e linhas na grade (basicamente a resolução)
        viewport_width, viewport_height, // tamanho da janela (em metros)
        viewport_distance, // distância da janela até o observador (em metros)
        bg_color // cor do background
    );

    // Definindo as propriedades de cada objeto
    let sphere1_radius = 0.5; // Raio em metros
    let sphere1_center = Vec3::new(-1.2, p0.y, p0.z - (viewport_distance + sphere1_radius)); // Coords. centro da esfera (metros)
    let sphere1_material = Material::new(
        Vec3::new(0.1, 0.0, 0.0), // Ambient
        Vec3::new(0.7, 0.0, 0.0), // Diffuse
        Vec3::new(0.3, 0.3, 0.3), // Specular
        500.0, // coeficiente de "brilho" ou "polimento"
    );

    let sphere2_radius = 0.5;
    let sphere2_center = Vec3::new(0.0, p0.y, p0.z - (viewport_distance + sphere2_radius));
    let sphere2_material = Material::new(
        Vec3::new(0.0, 0.1, 0.0),
        Vec3::new(0.0, 0.7, 0.0), 
        Vec3::new(0.3, 0.3, 0.3), 
        500.0,
    );

    let sphere3_radius = 0.5;
    let sphere3_center = Vec3::new(1.2, p0.y, p0.z - (viewport_distance + sphere3_radius));
    let sphere3_material = Material::new(
        Vec3::new(0.0, 0.0, 0.1), 
        Vec3::new(0.0, 0.0, 0.7), 
        Vec3::new(0.3, 0.3, 0.3), 
        500.0, 
    );
    
    let plane1_p0 = Vec3::new(0.0, -0.5, 0.0); // Ponto conhecido do plano
    let plane1_normal = Vec3::new(0.0, 1.0 ,0.0); // Normal do plano
    let plane1_material = Material::new(
        Vec3::new(0.1, 0.1, 0.1), 
        Vec3::new(0.4, 0.4, 0.4), 
        Vec3::new(0.0, 0.0, 0.0), 
        3.0, 
    );

    let plane2_p0 = Vec3::new(0.0, 0.0, -5.0); // Ponto conhecido do plano
    let plane2_normal = Vec3::new(0.0, 0.0 ,1.0); // Normal do plano
    let plane2_material = Material::new(
        Vec3::new(0.05, 0.05, 0.1), 
        Vec3::new(0.1, 0.1, 0.3), 
        Vec3::new(0.0, 0.0, 0.0), 
        3.0, 
    );
    
    // Definindo as propriedades das luzes
    let light1_pos = Vec3::new(-1.6, 0.8, 0.0);
    let light1_color = Vec3::new(1.0, 0.0, 0.0);
    let light1_intensity = 1.0;

    let light2_pos = Vec3::new(0.0, 0.8, 0.0);
    let light2_color = Vec3::new(0.0, 1.0, 0.0);
    let light2_intensity = 1.0;

    let light3_pos = Vec3::new(1.6, 0.8, 0.0);
    let light3_color = Vec3::new(0.0, 0.0, 1.0);
    let light3_intensity = 1.0;
    
    // Criando os objetos e as luzes
    let plane1 = Plane::new( plane1_p0, plane1_normal, plane1_material );
    let plane2 = Plane::new( plane2_p0, plane2_normal, plane2_material );
    let sphere1 = Sphere::new( sphere1_center, sphere1_radius, sphere1_material );
    let sphere2 = Sphere::new( sphere2_center, sphere2_radius, sphere2_material );
    let sphere3 = Sphere::new( sphere3_center, sphere3_radius, sphere3_material );
    let light1 = Light::new( light1_pos, light1_color, light1_intensity ); 
    let light2 = Light::new( light2_pos, light2_color, light2_intensity );
    let light3 = Light::new( light3_pos, light3_color, light3_intensity );
    let ambient_light = Vec3::new(1.0, 1.0, 1.0); // Luz ambiente
    
    // Adicionando os objetos na cena
    let mut scene = Scene::new(ambient_light);
    scene.add_shape(plane1);
    scene.add_shape(plane2);
    scene.add_shape(sphere1);
    scene.add_shape(sphere2);
    scene.add_shape(sphere3);
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
    
    // main loop do programa
    let mut frame_count = 0; // contador de FPS no terminal
    let mut last_time = Instant::now();
    'running: loop {
        // Seção de eventos e updates
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

        // Seção de draw
        camera.draw_scene(&mut canvas, &scene);
        
        // Contador de FPS
        frame_count += 1;
        if last_time.elapsed() >= Duration::new(1, 0) {
            println!("FPS: {frame_count}"); // printa o número de frames desenhados no último segundo (FPS)
            frame_count = 0;
            last_time = Instant::now();
        }
    }
}