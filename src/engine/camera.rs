#![allow(dead_code)]
use super::{Ray, Scene};
use super::shapes::Material;
use crate::utils::transform::rotation_around_axis;
use crate::utils::Vec3;
// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::surface::Surface;
use sdl2::video::WindowSurfaceRef;
// use sdl2::video::Window;
use std::{ptr, thread};
use std::sync::Arc;

pub struct Camera {
    pub pos: Vec3, // observador
    pub bg_color: Vec3,
    pub focal_distance: f64,
    pub coord_system: [Vec3; 3],
    viewport: Viewport, // janela
    draw_buffer: Vec<u8>,
}

impl Camera {
    #[inline]
    #[must_use]
    /// Cria uma nova câmera. \
    /// `pos`: posição da câmera na cena
    /// `n_cols`, `n_rows`: resolução X,Y da câmera (colunas e linhas no viewport) \
    /// `viewport_w`, `viewport_h`: Tamanho do viewport em metros \
    /// `viewport_distance`: Distância do viewport até o observador \
    /// `bg_color`: Cor do background
    /// `texture_creator`: TextureCreator para criar a textura interna de buffer da câmera
    pub fn new(pos: Vec3, n_cols: u32, n_rows: u32, viewport_w: f64, viewport_h: f64, focal_distance: f64, bg_color: Vec3) -> Camera {     
        Camera {
            pos, // posição do observador
            bg_color: bg_color.clamp(0.0, 1.0) * 255.0,
            focal_distance,
            coord_system: [Vec3::X, Vec3::Y, Vec3::Z],
            
            draw_buffer: vec![0; (n_cols * n_rows * 4) as usize],

            viewport: Viewport::new(
                Vec3::new(pos.x, pos.y, pos.z-focal_distance), // posição da janela em relação ao observador (0, 0, -d)
                viewport_w, viewport_h, // altura * largura da janela
                n_cols, n_rows, // número de colunas e linhas, basicamente a resolução da câmera.
            ),
        }
    }

    /// Desenha uma cena em um canvas com base nas especificações da câmera
    pub fn draw_scene(&mut self, scene: &Scene, mut surface: WindowSurfaceRef) {
        // Número de bytes no canvas (número de pixels * 3: RGB24)
        let num_bytes = self.viewport.cols * self.viewport.rows * 4;
        // Número de threads disponíveis * 3
        // (Nos meus testes usar o triplo de threads disponíveis tende a aumentar a eficiência por algum motivo)
        let num_threads = thread::available_parallelism().unwrap().get() as u32 * 3; 
        
        // Referências thread-safe
        let scene = Arc::new(&scene); // Cena
        let viewport = Arc::new(&self.viewport); // Viewport da câmera
        
        // Render multithread
        // (A câmera tem um array de pixels em formato RGB24. A gente divide esse buffer pra várias threads
        // e elas vão calcular os pixels em paralelo, acelerando o render.)
        thread::scope(|s| { // COMEÇO_RENDER_MULTITHREAD
        let mut lower_bound = 0;
        // Divide o array de buffer em chunks de tamanhos iguais pras threads
        for ppm_slice in self.draw_buffer.chunks_mut((num_bytes/num_threads) as usize) {
            // Clona as referências pesadas e os vetores leves para serem movidos para outra thread
            let scene = Arc::clone(&scene);
            let viewport = Arc::clone(&viewport);
            let pos = self.pos;
            let bg_color = self.bg_color;

            // Número de pixels que a thread vai desenhar
            let pixel_count = ppm_slice.len() / 4;
            
            s.spawn(move || {
                let mut ray = Ray::new(pos, Vec3::new(0.0,0.0,1.0)); // cria um raio partindo de p0 na direção d
                let mut mat: &Material;
                let mut rgb_counter = 0;
                
                for pixel in 0..pixel_count {
                    let row = (lower_bound + pixel) / (viewport.cols as usize);
                    let col = (lower_bound + pixel) % (viewport.cols as usize);
                    let dr = (((viewport.p00_coords) + (col as f64)*viewport.dx - (row as f64)*viewport.dy) - pos).normalize();
                    ray.dr = dr;

                    // Obtém o objeto mais próximo a colidir com o raio
                    let intersection = scene.get_intersection(&ray);

                    // se o raio não colide com nenhum objeto, desenha a cor do background e passa pro próximo pixel
                    if intersection.is_none() {
                        ppm_slice[rgb_counter] = bg_color.z as u8;
                        ppm_slice[rgb_counter + 1] = bg_color.y as u8;
                        ppm_slice[rgb_counter + 2] = bg_color.x as u8;
                        ppm_slice[rgb_counter + 3] = 255;
                        rgb_counter += 4;
                        continue;
                    }
                    
                    let (shape, t, n) = intersection.unwrap();
                    mat = shape.material(); // material do objeto
                    
                    // Calcula a cor do pixel de acordo com a iluminação
                    // intensidade da luz que chega no olho do observador (começa com a luz ambiente)
                    let mut ieye = mat.k_amb * scene.ambient_light;
                    let p_i = ray.at(t); // ponto de interseção
                    'lights: for light in &scene.lights {
                        // Checar se o objeto está na sombra de algum outro objeto
                        let light_ray = Ray::new(p_i, light.pos - p_i); // raio partindo de p_i até o ponto de luz
                        for s in &scene.shapes {
                            // Tem alguns problemas de iluminação com detecção de colisão consigo mesmo. Não sei ajeitar ainda.
                            if ptr::eq(s, shape) { continue; }

                            // se tem um objeto ENTRE p_i e a luz (não está atrás da luz ou atrás de p_i (0.0 < tl < 1.0))
                            // 0.0001 previne problemas com floating point precision
                            if let Some((tl, _)) = s.get_intersection(&light_ray) {
                                if 0.0001 < tl && tl < 1.0 { continue 'lights; }
                            }
                        }
                        
                        // Se o objeto não estiver na sombra...
                        let l = light_ray.dr.normalize(); // vetor unitário apontando na direção da luz
                    
                        let r = 2.0 * l.dot(n)*n - l; // vetor l refletido na normal
                        let nl = n.dot(l); // normal escalar l
                        let rv = r.dot(-dr); // r escalar v

                        // O check > 0.0 previne o bug de iluminação no "lado escuro da esfera"
                        if nl > 0.0 { ieye += mat.k_dif * nl * light.intensity; } // Reflexão difusa
                        if rv > 0.0 { ieye += mat.k_esp * rv.powf(mat.e) * light.intensity } // Reflexão especular
                    }
                    
                    // converte pra range de u8, etc.
                    ieye = ieye.clamp(0.0, 1.0) * 255.0;
                    
                    // salva o pixel no buffer da câmera
                    ppm_slice[rgb_counter] = ieye.z as u8;
                    ppm_slice[rgb_counter + 1] = ieye.y as u8;
                    ppm_slice[rgb_counter + 2] = ieye.x as u8;
                    ppm_slice[rgb_counter + 3] = 255;
                    rgb_counter += 4;
                    
                }
            });

            lower_bound += pixel_count;
        }
        }); // FIM_RENDER_MULTITHREAD

        surface.with_lock_mut(|pixels| {
            pixels.copy_from_slice(&self.draw_buffer);
        });
        surface.finish().unwrap();
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.pos = pos;
        self.viewport = Viewport::new(
            Vec3::new(pos.x, pos.y, pos.z-self.focal_distance), // posição da janela em relação ao observador (0, 0, -d)
            self.viewport.width, self.viewport.height, // altura * largura da janela
            self.viewport.cols, self.viewport.rows, // número de colunas e linhas, basicamente a resolução da câmera.
        );
    }

    pub fn add_position(&mut self, add: Vec3) {
        self.pos += add;
        self.viewport.add_position(add);
    }

    pub fn translate(&mut self, translation_vector: Vec3) {
        self.pos += translation_vector;
        self.viewport.pos += translation_vector;
        self.viewport.p00_coords += translation_vector;
        self.viewport.top_left_coords += translation_vector;
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f64) {
        let translation_vector = self.pos;
        let transformation_matrix = rotation_around_axis(axis, angle);

        self.pos -= translation_vector;
        self.viewport.pos -= translation_vector;
        self.viewport.p00_coords -= translation_vector;
        self.viewport.top_left_coords -= translation_vector;

        self.pos.transform(&transformation_matrix);
        for v in &mut self.coord_system { v.transform(&transformation_matrix); }
        self.viewport.pos.transform(&transformation_matrix);
        self.viewport.dx.transform(&transformation_matrix);
        self.viewport.dy.transform(&transformation_matrix);
        self.viewport.p00_coords.transform(&transformation_matrix);
        self.viewport.top_left_coords.transform(&transformation_matrix);

        self.pos += translation_vector;
        self.viewport.pos += translation_vector;
        self.viewport.p00_coords += translation_vector;
        self.viewport.top_left_coords += translation_vector;
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
    pub width: f64, pub height: f64,
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
    pub fn new(pos: Vec3, width: f64, height: f64, cols: u32, rows: u32) -> Viewport {
        let top_left_coords: Vec3 = Vec3::new(pos.x - width/2.0, pos.y + height/2.0, pos.z);
        let dx = Vec3::new(width/(cols as f64), 0.0, 0.0);
        let dy = Vec3::new(0.0, height/(rows as f64), 0.0);
        let p00_coords: Vec3 = top_left_coords + dx/2.0 - dy/2.0;
        
        Viewport {
            pos,
            height, width,
            rows, cols,

            dx, dy,
            top_left_coords, p00_coords,
        }
    }

    pub fn add_position(&mut self, add: Vec3) {
        self.top_left_coords += add;
        self.p00_coords += add;
    }
}