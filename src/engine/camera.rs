use std::f32::INFINITY;
use super::Ray;
use super::Scene;
use super::shapes::Material;
use super::shapes::Shape;
use crate::utils::Vec3;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::thread;
use std::sync::Arc;
use num_cpus;

#[derive(Clone, PartialEq)]
pub struct Camera {
    pub pos: Vec3, // observador
    pub bg_color: Vec3,
    viewport: Viewport, // janela
    draw_buffer: Vec<u8>
}

impl Camera {
    #[inline]
    #[must_use]
    /// Cria uma nova câmera. \
    /// `pos`: posição da câmera na cena
    /// `n_cols`, `n_rows`: resolução X,Y da câmera (colunas e linhas no viewport) \
    /// `viewport_w`, `viewport_h`: Tamanho do viewport em metros \
    /// `viewport_distance`: Distância do viewport até o observador \
    /// `bg_color` : Cor do background
    pub fn new(pos: Vec3, n_cols: u32, n_rows: u32, viewport_w: f32, viewport_h: f32, viewport_distance: f32, bg_color: Vec3) -> Camera {
        Camera {
            pos: pos, // posição do observador
            bg_color: bg_color.clamp(0.0, 1.0) * 255.0,
            viewport: Viewport::new(
                Vec3::new(pos.x, pos.y, pos.z-viewport_distance), // posição da janela em relação ao observador (0, 0, -d)
                viewport_w, viewport_h, // altura * largura da janela
                n_cols, n_rows, // número de colunas e linhas, basicamente a resolução da câmera.
            ),
            draw_buffer: vec![0; (n_cols * n_rows * 3) as usize]
        }
    }

    /// Desenha uma cena em um canvas com base nas especificações da câmera
    pub fn draw_scene(&mut self, canvas: &mut Canvas<Window>, scene: &Scene) {
        let num_pixels = self.viewport.cols * self.viewport.rows * 3;
        let num_threads = num_cpus::get() as u32 * 2;
        
        let bg_color = self.bg_color;
        let scene = Arc::new(&scene);
        let viewport = Arc::new(&self.viewport);
        let pos = Arc::new(self.pos);
        
        thread::scope(|s| {
        let mut lower_bound = 0;
        for ppm_slice in self.draw_buffer.chunks_mut((num_pixels/num_threads) as usize) {
            let scene = Arc::clone(&scene);
            let viewport = Arc::clone(&viewport);
            let pos = *Arc::clone(&pos);

            let chunk_size = ppm_slice.len() / (viewport.cols as usize) / 3;
            let upper_bound = lower_bound + chunk_size;
            
            s.spawn(move || {
                let mut ray = Ray::new(pos, Vec3::new(0.0,0.0,1.0)); // cria um raio partindo de p0 "atirado" na direção d
                let mut mat: &Material;
                let mut rgb_counter = 0;
                
                for row in lower_bound..upper_bound {
                    for col in 0..(viewport.cols as i32) {
                        let dr = (((viewport.p00_coords) + (col as f32)*viewport.dx - (row as f32)*viewport.dy) - pos).normalize();
                        ray.dr = dr;

                        let mut shape: Option<&Shape> = None;
                        let mut t = INFINITY;
                        for s in &scene.shapes {
                            let t_s = s.intersects(&ray);
                            // se o objeto colide com o raio, não está atrás do observador, e tá mais próximo que todo objeto testado até agr
                            if t_s > 0.0 && t_s < t {
                                shape = Some(s);
                                t = t_s;
                            }
                        }
                        if shape.is_none() { // se o raio não colide com nenhum objeto, desenha a cor do background passa pro próximo pixel
                            ppm_slice[rgb_counter] = bg_color.x as u8;
                            ppm_slice[rgb_counter] = bg_color.y as u8;
                            ppm_slice[rgb_counter] = bg_color.z as u8;
                            rgb_counter += 3;
                            continue;
                        }
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
                                // Skipa o cálculo se a interseção for consigo mesmo, previne uns bugs de reflexão especular inclusive
                                if s == shape { continue; }
                                let tl = s.intersects(&light_ray);
                                if tl < 1.0 && tl > 0.0001 { under_light = false; break; } // se tem um objeto ENTRE P_I E A LUZ 
                            }
                            
                            if under_light {
                                let n = shape.normal(&p_i); // vetor normal do objeto com o ponto p_i
                                let r = (2.0 * (l.dot(n)))*n - l; // vetor l refletido na normal
        
                                let nl = n.dot(l); // normal escalar l
                                let rv = r.dot(-dr); // r escalar v
        
                                // impede de desenhar a luz no "lado escuro da esfera"
                                if nl > 0.0 { idif = mat.k_dif * nl * light.color * light.intensity }
                                if rv > 0.0 { iesp = mat.k_esp * rv.powf(mat.e) * light.color * light.intensity }
                                
                                ieye += idif + iesp;
                            }
                        }
                        
                        ieye = ieye.clamp(0.0, 1.0) * 255.0;
                        
                        ppm_slice[rgb_counter] = ieye.x as u8;
                        ppm_slice[rgb_counter + 1] = ieye.y as u8;
                        ppm_slice[rgb_counter + 2] = ieye.z as u8;
                        rgb_counter += 3;
                    }
                }
            });

            lower_bound += chunk_size;
        }
        });

        // save_vec8_as_ppm(&final_buffer, viewport.cols as i32, viewport.rows as i32, 999).unwrap();

        let surface = Surface::from_data(
            &mut self.draw_buffer,
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
/// Janela através a qual o observador vai olhar \
/// `pos`: posição do Viewport (por enquanto vai estar em p0 - (0,0,d)) \
/// `width`, `height`: largura x altura da janela (em metros) \
/// `cols`, `rows`: número de colunas e linhas da grade (praticamente a resolução) \
/// `dx`, `dy`: tamanho x e y de cada quadrado \
/// `top_left_coords`: coordenadas da quina superior esquerda do frame \
/// `p00_coords`: coordenadas do quadrado 0,0 do frame
struct Viewport {
    pub pos: Vec3, 
    pub width: f32, pub height: f32,
    pub cols: u32, pub rows: u32,

    pub dx: Vec3, pub dy: Vec3,
    pub top_left_coords: Vec3,
    pub p00_coords: Vec3
}

impl Viewport { 
    #[inline]
    #[must_use]
    /// Cria um novo viewport. \
    /// `pos`: posição do Viewport (por enquanto vai estar em p0 - (0,0,d)) \
    /// `width`, `height`: largura x altura da janela (em metros) \
    /// `cols`, `rows`: número de colunas e linhas da grade (praticamente a resolução) \
    /// `dx`, `dy`: tamanho x e y de cada quadrado \
    /// `top_left_coords`: coordenadas da quina superior esquerda do frame \
    /// `p00_coords`: coordenadas do quadrado 0,0 do frame
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