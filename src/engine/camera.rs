#![allow(dead_code)]
use super::{Ray, Scene};
use super::Light;
use crate::utils::transform::rotation_around_axis;
use crate::utils::Vec3;
use sdl2::surface::Surface;
// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::surface::Surface;
use std::f64::consts::PI;
// use sdl2::video::Window;
use std::{ptr, thread};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub enum Projection {
    Perspective,
    Ortographic,
    Oblique
}

pub struct Camera<'a> {
    pub pos: Vec3, // observador
    pub coord_system: [Vec3; 3],
    pub focal_distance: f64,
    pub projection_type: Projection,
    pub obliqueness: Vec3,
    pub viewport: Viewport, // janela   
    pub sdl_surface: Surface<'a>,
}

impl <'a> Camera<'a> {
    #[inline]
    #[must_use]
    /// Cria uma nova câmera. \
    /// `pos`: posição da câmera na cena
    /// `n_cols`, `n_rows`: resolução X,Y da câmera (colunas e linhas no viewport) \
    /// `viewport_w`, `viewport_h`: Tamanho do viewport em metros \
    /// `viewport_distance`: Distância do viewport até o observador \
    /// `bg_color`: Cor do background
    /// `texture_creator`: TextureCreator para criar a textura interna de buffer da câmera
    pub fn new(pos: Vec3, n_cols: u32, n_rows: u32, viewport_w: f64, viewport_h: f64, focal_distance: f64) -> Camera<'a> {     
        let sdl_surface = Surface::new(n_cols, n_rows, sdl2::pixels::PixelFormatEnum::RGB888).unwrap();

        Camera {
            pos, // posição do observador
            focal_distance,
            coord_system: [Vec3::X, Vec3::Y, Vec3::Z],
            projection_type: Projection::Perspective,
            obliqueness: Vec3::new(0.0, 30.0, 0.0),
            
            sdl_surface,

            viewport: Viewport::new(
                Vec3::new(pos.x, pos.y, pos.z-focal_distance), // posição da janela em relação ao observador (0, 0, -d)
                viewport_w, viewport_h, // altura * largura da janela
                n_cols, n_rows, // número de colunas e linhas, basicamente a resolução da câmera.
            ),
        }
    }

    pub fn set_viewport_size(&mut self, width: f64, height: f64) {
        self.viewport = Viewport::new(
            Vec3::new(0.0, 0.0, self.pos.z-self.focal_distance), // posição da janela em relação ao observador (0, 0, -d)
            width, height, // altura * largura da janela
            self.viewport.cols, self.viewport.rows, // número de colunas e linhas, basicamente a resolução da câmera.
        );
        self.viewport.top_left_coords = self.camera_to_world(self.viewport.top_left_coords);
        self.viewport.p00 = self.camera_to_world(self.viewport.p00);
        self.viewport.pos = self.camera_to_world(self.viewport.pos);
        self.viewport.dx = self.coord_system[0] * self.viewport.dx.length();
        self.viewport.dy = self.coord_system[1] * self.viewport.dy.length();
    }

    pub fn world_to_camera(&self, point: Vec3) -> Vec3 {
        let point_translated = point - self.pos;
        let point_rotated = Vec3::new(
            point_translated.dot(self.coord_system[0]),
            point_translated.dot(self.coord_system[1]),
            point_translated.dot(self.coord_system[2]),
        );
        point_rotated
    }

    pub fn camera_to_world(&self, point: Vec3) -> Vec3 {
        let point_rotated = point.x * self.coord_system[0]
            + point.y * self.coord_system[1]
            + point.z * self.coord_system[2];
        let point_translated = point_rotated + self.pos;
        point_translated
    }

    
    pub fn set_focal_distance(&mut self, focal_distance: f64) {
        self.focal_distance = focal_distance;
        self.viewport = Viewport::new(
            Vec3::new(0.0, 0.0, -focal_distance), // posição da janela em relação ao observador (0, 0, -d)
            self.viewport.width, self.viewport.height, // altura * largura da janela
            self.viewport.cols, self.viewport.rows, // número de colunas e linhas, basicamente a resolução da câmera.
        );
        self.viewport.top_left_coords = self.camera_to_world(self.viewport.top_left_coords);
        self.viewport.p00 = self.camera_to_world(self.viewport.p00);
        self.viewport.pos = self.camera_to_world(self.viewport.pos);
        self.viewport.dx = self.coord_system[0] * self.viewport.dx.length();
        self.viewport.dy = self.coord_system[1] * self.viewport.dy.length();
    }

    /// Desenha uma cena em um canvas com base nas especificações da câmera
    pub fn draw_scene(&mut self, scene: &Scene) {
        // Número de bytes no canvas (número de pixels * 3: RGB24)
        let num_bytes = self.viewport.cols * self.viewport.rows * 4;
        // Número de threads disponíveis * 3
        // (Nos meus testes usar o triplo de threads disponíveis tende a aumentar a eficiência por algum motivo)
        let num_threads = thread::available_parallelism().unwrap().get() as u32 * 3; 
        
        // Referências thread-safe
        let scene = Arc::new(scene); // Cena
        let viewport = Arc::new(&self.viewport); // Viewport da câmera
        
        // Render multithread
        // (A câmera tem um array de pixels em formato RGB24. A gente divide esse buffer pra várias threads
        // e elas vão calcular os pixels em paralelo, acelerando o render.)
        let surface_pixels = self.sdl_surface.without_lock_mut().unwrap();
        thread::scope(|s| {
        let mut lower_bound = 0;
        // Divide o array de buffer em chunks de tamanhos iguais pras threads
        for ppm_slice in surface_pixels.chunks_mut((num_bytes/num_threads) as usize) {
            // Clona as referências pesadas e os vetores leves para serem movidos para outra thread
            let scene = Arc::clone(&scene);
            let viewport = Arc::clone(&viewport);
            let self_pos = self.pos;
            let bg_color = scene.bg_color;
            let projection_type = self.projection_type;
            let coord_system = self.coord_system;
            let obliqueness = self.obliqueness;

            // Número de pixels que a thread vai desenhar
            let pixel_count = ppm_slice.len() / 4;
            
            s.spawn(move || {
                let mut ray = match projection_type {
                    Projection::Perspective => {
                        Ray::new(self_pos, Vec3::NULL) // cria um raio partindo de p0 na direção d
                    }
                    Projection::Ortographic => {
                        Ray::new(viewport.p00, -coord_system[2])
                    }
                    Projection::Oblique => {
                        let mut dr = -coord_system[2];
                        if obliqueness.x != 0.0 {
                            dr.transform(&rotation_around_axis(coord_system[0], obliqueness.x.to_radians()));
                        }
                        if obliqueness.y != 0.0 {
                            dr.transform(&rotation_around_axis(coord_system[1], obliqueness.y.to_radians()));
                        }
                        if obliqueness.z != 0.0 {
                            dr.transform(&rotation_around_axis(coord_system[2], obliqueness.z.to_radians()));
                        }
                        Ray::new(viewport.p00, dr)
                    }
                };
                let mut rgb_counter = 0;
                
                for pixel in 0..pixel_count {
                    let row = (lower_bound + pixel) / (viewport.cols as usize);
                    let col = (lower_bound + pixel) % (viewport.cols as usize);
                    match projection_type {
                        Projection::Perspective => {
                            ray.dr = (((viewport.p00) + (col as f64)*viewport.dx - (row as f64)*viewport.dy) - self_pos).normalized();
                        }
                        Projection::Ortographic => {
                            ray.origin = (viewport.p00) + (col as f64)*viewport.dx - (row as f64)*viewport.dy;
                        }
                        Projection::Oblique => {
                            ray.origin = (viewport.p00) + (col as f64)*viewport.dx - (row as f64)*viewport.dy;
                        }
                    };

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
                    
                    let (shape, t, n, mat) = intersection.unwrap();
                    // mat = shape.material(); // material do objeto
                    
                    // Calcula a cor do pixel de acordo com a iluminação
                    // intensidade da luz que chega no olho do observador (começa com a luz ambiente)
                    let mut ieye = mat.k_amb * scene.ambient_light;
                    let p_i = ray.at(t); // ponto de interseção
                    'lights: for light in &scene.lights {
                        let ldr: Vec3;
                        let light_intensity: Vec3;
                        let mut is_directional= false;
                        match light {
                            Light::Point { pos, intensity } => {
                                ldr = *pos - p_i;
                                light_intensity = *intensity;
                            }
                            Light::Spotlight { pos, dr, angle, intensity } => {
                                ldr = *pos - p_i;
                                light_intensity = *intensity;
                                if dr.dot(ldr.normalized()) <= angle.cos() { continue 'lights; }
                            }
                            Light::Directional { dr, intensity } => {
                                ldr = *dr;
                                light_intensity = *intensity;
                                is_directional = true;
                            }
                        }

                        // Checar se o objeto está na sombra de algum outro objeto
                        let light_ray = Ray::new(p_i, ldr); // raio partindo de p_i até o ponto de luz
                        for s in &scene.shapes {
                            // Tem alguns problemas de iluminação com detecção de colisão consigo mesmo. Não sei ajeitar ainda.
                            if ptr::eq(s, shape) { continue; }

                            // se tem um objeto ENTRE p_i e a luz (não está atrás da luz ou atrás de p_i (0.0 < tl < 1.0))
                            // 0.0001 previne problemas com floating point precision
                            if let Some((tl, _, _)) = s.get_intersection(&light_ray) {
                                if 0.0001 < tl && (is_directional || tl < 0.9999) { continue 'lights; }
                            }
                        }
                        
                        // Se o objeto não estiver na sombra...
                        let l = light_ray.dr.normalized(); // vetor unitário apontando na direção da luz
                    
                        let r = 2.0 * l.dot(n)*n - l; // vetor l refletido na normal
                        let nl = n.dot(l); // normal escalar l
                        let rv = r.dot(-ray.dr); // r escalar v

                        // O check > 0.0 previne o bug de iluminação no "lado escuro da esfera"
                        if nl > 0.0 { ieye += mat.k_dif * nl * light_intensity; } // Reflexão difusa
                        if rv > 0.0 { ieye += mat.k_esp * rv.powf(mat.e) * light_intensity } // Reflexão especular
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
        });
    }

    #[must_use]
    /// índice do objeto, ponto de interseção, normal
    pub fn send_ray(&self, row: i32, col: i32, scene: &Scene) -> Option<(usize, Vec3, Vec3)> {
        let mut ray = match self.projection_type {
            Projection::Perspective => {
                Ray::new(self.pos, Vec3::new(0.0,0.0,1.0)) // cria um raio partindo de p0 na direção d
            }
            Projection::Ortographic => {
                Ray::new(self.viewport.p00, -self.coord_system[2])
            }
            Projection::Oblique => {
                Ray::new(self.viewport.p00, -self.coord_system[2])
            }
        };

        match self.projection_type {
            Projection::Perspective => {
                ray.dr = (((self.viewport.p00) + (col as f64)*self.viewport.dx - (row as f64)*self.viewport.dy) - self.pos).normalized();
            }
            Projection::Ortographic => {
                ray.origin = (self.viewport.p00) + (col as f64)*self.viewport.dx - (row as f64)*self.viewport.dy;
            }
            Projection::Oblique => {
                ray.origin = (self.viewport.p00) + (col as f64)*self.viewport.dx - (row as f64)*self.viewport.dy;
            }
        };

        let intersection = scene.get_intersection(&ray);
        match intersection {
            None => { None }
            Some( (shape, t, normal, _material) )=> {
                let mut counter: usize = 0;
                for s in &scene.shapes {
                    if ptr::eq(s, shape) { break; }
                    counter += 1;
                }
                Some( (counter, ray.at(t), normal) )
            }
        }

    }

    pub fn set_projection(&mut self, projection: Projection) {
        self.projection_type = projection;
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.translate(pos - self.pos);
    }

    pub fn translate(&mut self, translation_vector: Vec3) {
        self.pos += translation_vector;
        self.viewport.pos += translation_vector;
        self.viewport.p00 += translation_vector;
        self.viewport.top_left_coords += translation_vector;
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f64) {
        let translation_vector = self.pos;
        let transformation_matrix = rotation_around_axis(axis, angle);

        self.pos -= translation_vector;
        self.viewport.pos -= translation_vector;
        self.viewport.p00 -= translation_vector;
        self.viewport.top_left_coords -= translation_vector;

        self.pos.transform(&transformation_matrix);
        for v in &mut self.coord_system { v.transform(&transformation_matrix); }
        self.viewport.pos.transform(&transformation_matrix);
        self.viewport.dx.transform(&transformation_matrix);
        self.viewport.dy.transform(&transformation_matrix);
        self.viewport.p00.transform(&transformation_matrix);
        self.viewport.top_left_coords.transform(&transformation_matrix);

        self.pos += translation_vector;
        self.viewport.pos += translation_vector;
        self.viewport.p00 += translation_vector;
        self.viewport.top_left_coords += translation_vector;
    }

    pub fn look_at(&mut self, point: Vec3, up: Vec3) {
        let z_axis = (point - self.pos).normalized(); // z axis of where i'm lookin at
        let x_axis = up.cross(z_axis).normalized(); // mia direita segundo o vetor up
        // let y_axis = z_axis.cross(x_axis);
    
        let current_z_axis = self.coord_system[2];
        let rotation_axis = current_z_axis.cross(z_axis).normalized();
        let rotation_angle = current_z_axis.angle(z_axis);
    
        self.rotate(rotation_axis, rotation_angle+PI);
    
        let current_x_axis = self.coord_system[0];
        let rotation_axis = current_x_axis.cross(x_axis).normalized();
        let rotation_angle = current_x_axis.angle(x_axis);
    
        self.rotate(rotation_axis, rotation_angle+PI);
    }
}


#[derive(Clone, PartialEq)]
/// Janela através a qual o observador vai olhar \
/// `pos`: posição do Viewport (por enquanto vai estar em p0 - (0,0,d)) \
/// `width`, `height`: largura x altura da janela (em metros) \
/// `cols`, `rows`: número de colunas e linhas da grade (praticamente a resolução) \
/// `dx`, `dy`: tamanho x e y de cada quadrado \
/// `top_left_coords`: coordenadas da quina superior esquerda do frame \
/// `p00`: coordenadas do quadrado 0,0 do frame
pub struct Viewport {
    pub pos: Vec3, 
    pub width: f64, pub height: f64,
    pub cols: u32, pub rows: u32,

    pub dx: Vec3, pub dy: Vec3,
    pub top_left_coords: Vec3,
    pub p00: Vec3
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
    /// `p00`: coordenadas do quadrado 0,0 do frame
    pub fn new(pos: Vec3, width: f64, height: f64, cols: u32, rows: u32) -> Viewport {
        let top_left_coords: Vec3 = Vec3::new(pos.x - width/2.0, pos.y + height/2.0, pos.z);
        let dx = Vec3::new(width/(cols as f64), 0.0, 0.0);
        let dy = Vec3::new(0.0, height/(rows as f64), 0.0);
        let p00: Vec3 = top_left_coords + dx/2.0 - dy/2.0;
        
        Viewport {
            pos, // centro do viewport
            height, width,
            rows, cols,

            dx, dy,
            top_left_coords, p00,
        }
    }
}
