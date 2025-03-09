mod engine;
mod utils;
mod scenes;
mod user_interface;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use user_interface::make_ui;
use utils::Vec3;
use std::{f64::consts::PI, time::{Duration, Instant}};
use imgui::Context;
use imgui_glow_renderer::{
    glow,
    // glow::HasContext,
    AutoRenderer,
};
use imgui_sdl2_support::SdlPlatform;
// use sdl2::video::GLProfile;
use sdl2::video::Window;

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn main() {
    let (mut scene, mut camera, window_width, window_height) = scenes::beach();
    let scale = 2.0;

    // Inicializando SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap(); // cuida dos eventos como teclado mouse etc.
    let window = video_subsystem // a janela do computador em si
        .window("CG1 - engine", ((window_width as f64)*scale) as u32, ((window_height as f64)*scale) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    // IMGUI
    let gl_context_gui = window.gl_create_context().unwrap();
    // window.subsystem().gl_set_swap_interval(1).unwrap();

    let gl = glow_context(&window);
    let mut imgui = Context::create();
    
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    imgui.fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
    // create platform and renderer
    let mut platform = SdlPlatform::new(&mut imgui);
    let mut renderer = AutoRenderer::new(gl, &mut imgui).unwrap();
    // END_IMGUI

    let mut selected_shape: Option<usize> = None;
    
    // main loop do programa
    let mut frame_count = 0; // contador de FPS no terminal
    let mut last_time = Instant::now();
    'running: loop {
        // Seção de eventos e updates
        for event in event_pump.poll_iter() {
            // pass all events to imgui platfrom
            platform.handle_event(&mut imgui, &event);
            let cdx = camera.coord_system[0];
            // let cdy = camera.coord_system[1];
            let cdz = camera.coord_system[2];
            let angle_step = PI/2.0/10.0;
            match event {
                // muda a posição da bola em 10cm pra cada lado pelas setas do teclado
                // setas = eixos x,y // W,S = eixo z
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { camera.translate(-0.1*cdz); } // FRONT
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { camera.translate(0.1*cdz); } // BACK
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { camera.translate(-0.1*cdx); } // LEFT
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { camera.translate(0.1*cdx); } // RIGHT
                Event::KeyDown { keycode: Some(Keycode::SPACE), .. } => { camera.translate(0.1*Vec3::Y); } // UP
                Event::KeyDown { keycode: Some(Keycode::LSHIFT), .. } => { camera.translate(-0.1*Vec3::Y); } // DOWN
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => { camera.rotate(Vec3::Y, angle_step); } // ROTATE LEFT
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => { camera.rotate(Vec3::Y, -angle_step); } // ROTATE RIGHT
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => { camera.rotate(cdx, angle_step); } // ROTATE UP
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => { camera.rotate(cdx, -angle_step); } // ROTATE DOWN
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => { camera.rotate(cdz, angle_step); } // ROLL LEFT
                Event::KeyDown { keycode: Some(Keycode::E), .. } => { camera.rotate(cdz, -angle_step); } // ROLL RIGHT
                // FOV
                Event::KeyDown { keycode: Some(Keycode::LEFTBRACKET), .. } => { camera.set_focal_distance(camera.focal_distance + 0.1); }
                Event::KeyDown { keycode: Some(Keycode::RIGHTBRACKET), .. } => { camera.set_focal_distance(camera.focal_distance - 0.1); }
                // PROJECTION
                Event::KeyDown { keycode: Some(Keycode::F1), .. } => { camera.set_projection(engine::camera::Projection::Perspective); }
                Event::KeyDown { keycode: Some(Keycode::F2), .. } => { camera.set_projection(engine::camera::Projection::Ortographic); }
                Event::KeyDown { keycode: Some(Keycode::F3), .. } => { camera.set_projection(engine::camera::Projection::Oblique); }
                // MOUSE CLICK
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Right => {
                            if let Some((s, _, _)) = camera.send_ray(y/scale as i32, x/scale as i32, &scene) {
                                selected_shape = Some(s);
                            }
                        }
                        MouseButton::Middle => {
                            if let Some((_, p, _)) = camera.send_ray(y/scale as i32, x/scale as i32, &scene) {
                                camera.look_at(p, Vec3::Y);
                            }
                        }
                        _ => {}
                    }
                }
                // esc pra sair do programa
                Event::Quit{ .. } => break 'running,
                _ => {}
            }
            // println!("{:?}", camera.pos);
        }
        
        // render scene
        camera.draw_scene(&scene);
        let mut window_surface = window.surface(&event_pump).unwrap();
        let window_rect = window_surface.rect();
        camera.sdl_surface.blit_scaled(camera.sdl_surface.rect(), &mut window_surface, window_rect).unwrap();
        window_surface.finish().unwrap();
        
        // create imgui UI
        platform.prepare_frame(&mut imgui, &window, &event_pump);
        let ui = imgui.new_frame();
        make_ui(ui, &mut scene, &mut camera, &mut selected_shape);
        let draw_data = imgui.render();
        

        // render UI
        window.gl_make_current(&gl_context_gui).unwrap();
        renderer.render(draw_data).unwrap();
        window.gl_swap_window();

        // Contador de FPS
        frame_count += 1;
        if last_time.elapsed() >= Duration::new(1, 0) {
            println!("FPS: {frame_count}"); // printa o número de frames desenhados no último segundo (FPS)
            frame_count = 0;
            last_time = Instant::now();
        }
    }
}
