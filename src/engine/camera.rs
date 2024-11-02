use std::f32::INFINITY;
use super::Ray;
use super::Scene;
use super::shapes::Material;
use super::shapes::Shape;
use crate::utils::Vec3;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use num_cpus;

#[derive(Clone, PartialEq)]
pub struct Camera {
    pub pos: Vec3, // observador
    pub bg_color: Color,
    viewport: Viewport // janela
}

impl Camera {
    #[inline]
    #[must_use]
    pub fn new(pos: Vec3, n_cols: u32, n_rows: u32, viewport_w: f32, viewport_h: f32, viewport_distance: f32, bg_color: Color) -> Camera {
        Camera {
            pos: pos, // posição do observador
            bg_color: bg_color,
            viewport: Viewport::new(
                Vec3::new(pos.x, pos.y, pos.z-viewport_distance), // posição da janela em relação ao observador (0, 0, -d)
                viewport_w, viewport_h, // altura * largura da janela
                n_cols, n_rows, // número de colunas e linhas, basicamente a resolução da câmera.
            )
        }
    }

    // draws entire scene
    pub fn draw_scene(&mut self, canvas: &mut Canvas<Window>, scene: &Scene) {
        canvas.set_draw_color(self.bg_color);
        canvas.clear();
        let scene = Arc::new(scene.clone());
        let viewport = Arc::new(self.viewport.clone());
        let pos = Arc::new(self.pos.clone());

        
        let (transmitter, receiver) = mpsc::channel();

        let num_threads = num_cpus::get() as i32 * 2;
        for thread_n in 0..num_threads {
            let scene = Arc::clone(&scene);
            let viewport = Arc::clone(&viewport);
            let pos = Arc::clone(&pos);
            let transmitter = transmitter.clone();
            
            let chunk_size = (viewport.rows as f32) / (num_threads as f32);
            let lower_bound = (chunk_size * (thread_n as f32)).round() as i32;
            let upper_bound = (chunk_size * ((thread_n+1) as f32)).round() as i32;
            let num_pixels = (upper_bound - lower_bound) * viewport.cols as i32 * 3;
            
            thread::spawn(move || {
                let pos = *pos;
                let mut ray = Ray::new(pos, Vec3::new(0.0,0.0,1.0)); // cria um raio partindo de p0 "atirado" na direção d
                let mut mat: &Material;
                let mut cu: Vec<u8> = vec![0; num_pixels as usize];
                let mut cu_counter = 0;
                
                for row in lower_bound..upper_bound { // TODO: thread_n
                    for col in 0..(viewport.cols as i32) {
                        let dr = (((viewport.p00_coords) + (col as f32)*viewport.dx - (row as f32)*viewport.dy) - pos).normalize();
                        ray.dr = dr;

                        let mut shape: Option<&Shape> = None;
                        let mut t = INFINITY;
                        for s in &scene.shapes {
                            let t_s = s.intersects(&ray);
                            // se o objeto colid com o raio, não está atrás do observador, e tá mais próximo que todo objeto testado até agr
                            if t_s > 0.0 && t_s < t {
                                shape = Some(s);
                                t = t_s;
                            }
                        }
                        if shape.is_none() { cu_counter += 3; continue; } // se o raio não colide com nenhum objeto, passa pro próximo pixel
                        let shape = shape.unwrap();
                        mat = shape.material(); // material do objeto
                        let mut ieye = mat.k_amb * scene.ambient_light; // intensidade da luz que chega no olho do observador
                        let p_i = ray.at(t); // ponto de interseção
                        
                        for light in &scene.lights {
                            let l = (light.pos - p_i).normalize(); // vetor apontando na direção da luz
                            let mut idif = Vec3::NULL; // cor vindo de reflexão difusa
                            let mut iesp = Vec3::NULL; // cor vindo de reflexão especular
                            let mut under_light = true;
        
                            let light_ray = Ray::new(p_i, light.pos - p_i); // raio partindo de p_i até a posição da luz
                            for s in &scene.shapes {
                                if s == shape { continue; }
                                let tl = s.intersects(&light_ray);
                                if tl < 1.0 && tl > 0.0001 { under_light = false; break; } // se tem um objeto ENTRE P_I E A LUZ 
                            }
                            
                            if under_light {
                                let n = shape.normal(&p_i); // vetor normal do objeto com o ponto p_i
                                let r = (2.0 * (l.dot(n)))*n - l; // vetor l refletido na normal
        
                                let nl = n.dot(l); // normal escalar l
                                let rv = r.dot(-dr); // r escalar l
        
                                // impede de desenhar a luz no "lado escuro da esfera"
                                if nl > 0.0 { idif = mat.k_dif * nl * light.color * light.intensity }
                                if rv > 0.0 { iesp = mat.k_esp * rv.powf(mat.e) * light.color * light.intensity }
                                
                                ieye += idif + iesp;
                            }
                        }
                            
                        cu[cu_counter] = (ieye.x * 255.0) as u8;
                        cu[cu_counter + 1] = (ieye.y * 255.0) as u8;
                        cu[cu_counter + 2] = (ieye.z * 255.0) as u8;
                        cu_counter += 3;
                    }
                }
                transmitter.send((thread_n, cu, upper_bound-lower_bound)).unwrap();
            });
        }
        drop(transmitter);

        let mut final_buffer: Vec<u8> = Vec::new();
        let mut buffers: Vec<Vec<u8>> = vec![Vec::new(); num_threads as usize - 1];
        for message in receiver {
            if message.0 == 0 { final_buffer = message.1; }
            else { buffers[message.0 as usize - 1] = message.1 }
        }
        for buffer in buffers { 
            final_buffer.extend(buffer.iter());
        }
        
        // save_vec8_as_ppm(&final_buffer, viewport.cols as i32, viewport.rows as i32, 999).unwrap();

        let surface = Surface::from_data(
            &mut final_buffer,
            viewport.cols, viewport.rows,
            viewport.cols * 3, 
            sdl2::pixels::PixelFormatEnum::RGB24
        ).unwrap();

        let texture_creator = canvas.texture_creator();
        let texture_from_surface = texture_creator.create_texture_from_surface(surface).expect("oh my fucking god");
        canvas.copy(&texture_from_surface, None, Some(Rect::new(0, 0, viewport.cols, viewport.rows))).unwrap();
        canvas.present();
    }
}


#[derive(Clone, PartialEq)]
// Janela através a qual o observador vai olhar
struct Viewport {
    pub pos: Vec3, // posição do Viewport (vai sempre estar em p0 - (0,0,d))
    pub width: f32, pub height: f32, // largura x altura do quadro (em metros)
    pub cols: u32, pub rows: u32, // número de colunas e linhas do quadro (praticamente a resolução)

    pub dx: Vec3, pub dy: Vec3, // tamanho x e y de cada quadrado
    pub top_left_coords: Vec3, // coordenadas da quina superior esquerda do frame
    pub p00_coords: Vec3 // coordenadas do quadrado 0,0 do frame
}

impl Viewport { 
    #[inline]
    #[must_use]
    pub fn new(pos: Vec3, width: f32, height: f32, cols: u32, rows: u32) -> Viewport {
        let top_left_coords: Vec3 = Vec3::new(pos.x - width/2.0, pos.y + height/2.0, pos.z);
        let dx = Vec3::new(width/(cols as f32), 0.0, 0.0);
        let dy = Vec3::new(0.0, height/(rows as f32), 0.0);
        let p00_coords: Vec3 = top_left_coords + dx/2.0 - dy/2.0;
        
        Viewport {
            pos,
            height, width,
            rows, cols,

            dx, dy,
            top_left_coords, p00_coords,
        }
    }
}