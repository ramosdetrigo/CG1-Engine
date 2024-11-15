use std::f32::INFINITY;
use super::{Ray, Scene};
use super::shapes::{Shape, Material};
use crate::utils::Vec3;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::render::Texture;
use sdl2::video::{Window, WindowContext};
use std::thread;
use std::sync::Arc;

pub struct Camera <'a> {
    pub pos: Vec3, // observador
    pub bg_color: Vec3,
    texture: Texture<'a>,
    viewport: Viewport, // janela
    draw_buffer: Vec<u8>
}

impl <'a> Camera <'a> {
    #[inline]
    #[must_use]
    /// Cria uma nova câmera. \
    /// `pos`: posição da câmera na cena
    /// `n_cols`, `n_rows`: resolução X,Y da câmera (colunas e linhas no viewport) \
    /// `viewport_w`, `viewport_h`: Tamanho do viewport em metros \
    /// `viewport_distance`: Distância do viewport até o observador \
    /// `bg_color` : Cor do background
    pub fn new(pos: Vec3, n_cols: u32, n_rows: u32, viewport_w: f32, viewport_h: f32, viewport_distance: f32, bg_color: Vec3, texture_creator: &'a TextureCreator<WindowContext>) -> Camera <'a> {
        Camera {
            pos: pos, // posição do observador
            bg_color: bg_color.clamp(0.0, 1.0) * 255.0,
            texture: texture_creator.create_texture(
                sdl2::pixels::PixelFormatEnum::RGB24,
                sdl2::render::TextureAccess::Streaming,
                n_cols, n_rows
            ).unwrap(),
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
        // Número de bytes no canvas (número de pixels * 3: RGB24)
        let num_bytes = self.viewport.cols * self.viewport.rows * 3;
        // Número de threads disponíveis * 2
        // (Nos meus testes usar o dobro de threads disponíveis tende a aumentar a eficiência por algum motivo)
        let num_threads = thread::available_parallelism().unwrap().get() as u32 * 2; 
        
        // Referências thread-safe
        let scene = Arc::new(&scene); // Cena
        let viewport = Arc::new(&self.viewport); // Viewport da câmera
        
        // Render multithread
        // (A câmera tem um array de pixels em formato RGB24. A gente divide esse buffer pra várias threads
        // e elas vão calcular os pixels em paralelo, acelerando o render.)
        thread::scope(|s| {
        let mut lower_bound = 0;
        for ppm_slice in self.draw_buffer.chunks_mut((num_bytes/num_threads) as usize) {
            // Clona as referências pesadas e os vetores leves para serem movidos para outra thread
            let scene = Arc::clone(&scene);
            let viewport = Arc::clone(&viewport);
            let pos = self.pos;
            let bg_color = self.bg_color;

            // Número de linhas que a thread vai desenhar (range lower_bound..upper_bound)
            let num_rows = ppm_slice.len() / (viewport.cols as usize) / 3;
            let upper_bound = lower_bound + num_rows;
            
            s.spawn(move || {
                let mut ray = Ray::new(pos, Vec3::new(0.0,0.0,1.0)); // cria um raio partindo de p0 "atirado" na direção d
                let mut mat: &Material;
                let mut rgb_counter = 0;
                
                for row in lower_bound..upper_bound {
                    for col in 0..(viewport.cols as i32) {
                        let dr = (((viewport.p00_coords) + (col as f32)*viewport.dx - (row as f32)*viewport.dy) - pos).normalize();
                        ray.dr = dr;

                        // Obtém o objeto mais próximo a colidir com o raio
                        let mut shape: Option<&Shape> = None;
                        let mut t = INFINITY;
                        for s in &scene.shapes {
                            let t_candidate = s.intersects(&ray);
                            // se o objeto colide com o raio, não está atrás do observador, e tá mais próximo que todo objeto testado até agr
                            if t_candidate > 0.0 && t_candidate < t {
                                shape = Some(s);
                                t = t_candidate;
                            }
                        }
                        // se o raio não colide com nenhum objeto, desenha a cor do background e passa pro próximo pixel
                        if shape.is_none() {
                            ppm_slice[rgb_counter] = bg_color.x as u8;
                            ppm_slice[rgb_counter] = bg_color.y as u8;
                            ppm_slice[rgb_counter] = bg_color.z as u8;
                            rgb_counter += 3;
                            continue;
                        }
                        let shape = shape.unwrap();
                        mat = shape.material(); // material do objeto
                        // intensidade da luz que chega no olho do observador (começa como só a luz ambiente)
                        let mut ieye = mat.k_amb * scene.ambient_light;
                        let p_i = ray.at(t); // ponto de interseção
                        
                        // Desenha o pixel acordo com a iluminação
                        for light in &scene.lights {
                            let l = (light.pos - p_i).normalize(); // vetor apontando na direção da luz
                            let mut idif = Vec3::NULL; // cor vindo de reflexão difusa
                            let mut iesp = Vec3::NULL; // cor vindo de reflexão especular
                            let mut under_light = true;
        
                            // raio partindo de p_i até o ponto de luz
                            // (o dr não normalizado ajuda a checar se um objeto não está atrás do ponto de luz)
                            let light_ray = Ray::new(p_i, light.pos - p_i);

                            // Checar se o objeto está na sombra de algum outro objeto
                            for s in &scene.shapes {
                                // Skipa o cálculo se a interseção for consigo mesmo, previne uns bugs de reflexão especular inclusive
                                // (This is a problem for future me ! ;D)
                                if s == shape { continue; }

                                let tl = s.intersects(&light_ray);
                                // se tem um objeto ENTRE p_i e a luz (não está atrás da luz ou atrás de p_i (0.0001 < tl < 1.0))
                                // 0.0001 previne problemas com floating point precision
                                if tl < 1.0 && tl > 0.0001 { under_light = false; break; }
                            }
                            
                            // Se o objeto não estiver na sombra...
                            if under_light {
                                let n = shape.normal(p_i); // vetor normal do objeto com o ponto p_i
                                let r = (2.0 * (l.dot(n)))*n - l; // vetor l refletido na normal
        
                                let nl = n.dot(l); // normal escalar l
                                let rv = r.dot(-dr); // r escalar v
        
                                // impede de desenhar a luz no "lado escuro da esfera"
                                if nl > 0.0 { idif = mat.k_dif * nl * light.color * light.intensity }
                                if rv > 0.0 { iesp = mat.k_esp * rv.powf(mat.e) * light.color * light.intensity }
                                
                                ieye += idif + iesp;
                            }
                        }
                        
                        // converte pra RGB24 
                        ieye = ieye.clamp(0.0, 1.0) * 255.0;
                        
                        // salva o pixel no slice do buffer da câmera
                        ppm_slice[rgb_counter] = ieye.x as u8;
                        ppm_slice[rgb_counter + 1] = ieye.y as u8;
                        ppm_slice[rgb_counter + 2] = ieye.z as u8;
                        rgb_counter += 3;
                    }
                }
            });

            lower_bound += num_rows;
        }
        });

        self.texture.update(None, &self.draw_buffer, (viewport.cols*3) as usize).unwrap();
        // E desenha a textura no canvas
        canvas.copy(&self.texture, None, Some(Rect::new(0, 0, viewport.cols, viewport.rows))).unwrap();

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