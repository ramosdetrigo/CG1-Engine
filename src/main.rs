mod engine;
mod utils;
mod scenes;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use utils::Vec3;
use std::time::{Duration, Instant};
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
    #[allow(unused_mut)]
    let (mut scene, mut camera, window_width, window_height) = scenes::sphere_test();
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
    let gl_context_engine = window.gl_create_context().unwrap();
    let gl_context_gui = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context_gui).unwrap();
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
            match event {
                // muda a posição da bola em 10cm pra cada lado pelas setas do teclado
                // setas = eixos x,y // W,S = eixo z
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { camera.translate(-0.1*cdz); } // FRONT
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { camera.translate(0.1*cdz); } // BACK
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { camera.translate(-0.1*cdx); } // LEFT
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { camera.translate(0.1*cdx); } // RIGHT
                Event::KeyDown { keycode: Some(Keycode::SPACE), .. } => { camera.translate(0.1*Vec3::Y); } // UP
                Event::KeyDown { keycode: Some(Keycode::LSHIFT), .. } => { camera.translate(-0.1*Vec3::Y); } // DOWN
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => { camera.rotate(Vec3::Y, 0.1); } // ROTATE LEFT
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => { camera.rotate(Vec3::Y, -0.1); } // ROTATE RIGHT
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => { camera.rotate(cdx, 0.1); } // ROTATE UP
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => { camera.rotate(cdx, -0.1); } // ROTATE DOWN
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => { camera.rotate(cdz, 0.1); } // ROLL LEFT
                Event::KeyDown { keycode: Some(Keycode::E), .. } => { camera.rotate(cdz, -0.1); } // ROLL RIGHT
                // FOV
                Event::KeyDown { keycode: Some(Keycode::EQUALS), .. } => { camera.set_fov(camera.fov - 10.0); }
                Event::KeyDown { keycode: Some(Keycode::MINUS), .. } => { camera.set_fov(camera.fov + 10.0); }
                Event::KeyDown { keycode: Some(Keycode::LEFTBRACKET), .. } => { camera.set_focal_distance(camera.focal_distance + 0.1); }
                Event::KeyDown { keycode: Some(Keycode::RIGHTBRACKET), .. } => { camera.set_focal_distance(camera.focal_distance - 0.1); }
                Event::KeyDown { keycode: Some(Keycode::RETURN), .. } => { camera.set_viewport_size(1.6, 0.9); }
                // PROJECTION
                Event::KeyDown { keycode: Some(Keycode::NUM_1), .. } => { camera.set_projection(engine::camera::Projection::Perspective); }
                Event::KeyDown { keycode: Some(Keycode::NUM_2), .. } => { camera.set_projection(engine::camera::Projection::Ortographic); }
                Event::KeyDown { keycode: Some(Keycode::NUM_3), .. } => { camera.set_projection(engine::camera::Projection::Oblique); }
                // MOUSE CLICK
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            if let Some((ray, t, _)) = camera.send_ray(y/scale as i32, x/scale as i32, &scene) {
                                let p = ray.at(t);
                                camera.look_at(p, Vec3::Y);
                            }
                        }
                        _ => {}
                    }
                }
                // esc pra sair do programa
                Event::Quit{ .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
            // println!("{:?}", camera.pos);
        }
        
        // Seção de draw
        window.gl_make_current(&gl_context_engine).unwrap();
        
        camera.draw_scene(&scene);

        
        let mut window_surface = window.surface(&event_pump).unwrap();
        let window_rect = window_surface.rect();
        camera.sdl_surface.blit_scaled(camera.sdl_surface.rect(), &mut window_surface, window_rect).unwrap();
        window_surface.finish().unwrap();
        
        // imgui
        if false {
            window.gl_make_current(&gl_context_gui).unwrap();
            platform.prepare_frame(&mut imgui, &window, &event_pump);
            let ui = imgui.new_frame();
            /* create imgui UI here */
            // ui.show_demo_window(&mut true);
            ui.show_about_window(&mut false);
            /* render */
            let draw_data = imgui.render();
            renderer.render(draw_data).unwrap();
            window.gl_swap_window();
        }


        // Contador de FPS
        frame_count += 1;
        if last_time.elapsed() >= Duration::new(1, 0) {
            println!("FPS: {frame_count}"); // printa o número de frames desenhados no último segundo (FPS)
            frame_count = 0;
            last_time = Instant::now();
        }
    }
}
