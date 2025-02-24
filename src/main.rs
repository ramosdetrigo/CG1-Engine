mod engine;
mod utils;
mod scenes;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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
    let (mut scene, mut camera, window_width, window_height) = scenes::cone_test();
    let scale = 1.0; // TODO: fix scaling

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
    imgui
        .fonts()
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
            match event {
                // muda a posição da bola em 10cm pra cada lado pelas setas do teclado
                // setas = eixos x,y // W,S = eixo z
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => { scene.lights[0].pos.x += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => { scene.lights[0].pos.x -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => { scene.lights[0].pos.z -= 0.1; }
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => { scene.lights[0].pos.z += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { scene.lights[0].pos.y += 0.1; }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { scene.lights[0].pos.y -= 0.1; }
                // esc pra sair do programa
                Event::Quit{ .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Seção de draw
        window.gl_make_current(&gl_context_engine).unwrap();
        let surface = window.surface(&event_pump).unwrap();
        camera.draw_scene_to_canvas(&scene, surface);
        
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
