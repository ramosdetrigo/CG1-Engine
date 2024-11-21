use super::{Ray, Scene};
use super::shapes::{Shape, Material};
use crate::utils::Vec3;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::thread;
use std::sync::Arc;

pub struct Camera {
    pub pos: Vec3, // observador
    pub bg_color: Vec3,
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
    pub fn new(pos: Vec3, n_cols: u32, n_rows: u32, viewport_w: f32, viewport_h: f32, viewport_distance: f32, bg_color: Vec3) -> Camera {     
        Camera {
            pos, // posição do observador
            bg_color: bg_color.clamp(0.0, 1.0) * 255.0,

            
            draw_buffer: vec![0; (n_cols * n_rows * 3) as usize],

            viewport: Viewport::new(
                Vec3::new(pos.x, pos.y, pos.z-viewport_distance), // posição da janela em relação ao observador (0, 0, -d)
                viewport_w, viewport_h, // altura * largura da janela
                n_cols, n_rows, // número de colunas e linhas, basicamente a resolução da câmera.
            ),
        }
    }

    /// Desenha uma cena em um canvas com base nas especificações da câmera
    pub fn draw_scene_to_canvas(&mut self, scene: &Scene, canvas: &mut Canvas<Window>) {
        // Número de bytes no canvas (número de pixels * 3: RGB24)
        let num_bytes = self.viewport.cols * self.viewport.rows * 3;
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
            let pixel_count = ppm_slice.len() / 3;
            
            s.spawn(move || {
                let mut ray = Ray::new(pos, Vec3::new(0.0,0.0,1.0)); // cria um raio partindo de p0 na direção d
                let mut mat: &Material;
                let mut rgb_counter = 0;
                
                for pixel in 0..pixel_count {
                    let row = (lower_bound + pixel) / (viewport.cols as usize);
                    let col = (lower_bound + pixel) % (viewport.cols as usize);
                    let dr = (((viewport.p00_coords) + (col as f32)*viewport.dx - (row as f32)*viewport.dy) - pos).normalize();
                    ray.dr = dr;

                    // Obtém o objeto mais próximo a colidir com o raio
                    let mut shape: Option<&Shape> = None;
                    let mut t = f32::INFINITY;
                    let mut n = Vec3::NULL;
                    for s in &scene.shapes {
                        let (t_candidate, n_candidate) = s.intersects(&ray);
                        // se o objeto colide com o raio, não está atrás do observador, e tá mais próximo que todo objeto testado até agr
                        if t_candidate > 0.0 && t_candidate < t {
                            shape = Some(s);
                            t = t_candidate;
                            n = n_candidate;
                        }
                    }
                    // se o raio não colide com nenhum objeto, desenha a cor do background e passa pro próximo pixel
                    if shape.is_none() {
                        ppm_slice[rgb_counter] = bg_color.x as u8;
                        ppm_slice[rgb_counter + 1] = bg_color.y as u8;
                        ppm_slice[rgb_counter + 2] = bg_color.z as u8;
                        rgb_counter += 3;
                        continue;
                    }
                    let shape = shape.unwrap();
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
                            if s == shape { continue; }

                            let tl = s.intersects(&light_ray).0;
                            // se tem um objeto ENTRE p_i e a luz (não está atrás da luz ou atrás de p_i (0.0 < tl < 1.0))
                            // 0.0001 previne problemas com floating point precision
                            if 0.0001 < tl && tl < 1.0 { continue 'lights; }
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
                    ppm_slice[rgb_counter] = ieye.x as u8;
                    ppm_slice[rgb_counter + 1] = ieye.y as u8;
                    ppm_slice[rgb_counter + 2] = ieye.z as u8;
                    rgb_counter += 3;
                }
            });

            lower_bound += pixel_count;
        }
        }); // FIM_RENDER_MULTITHREAD

        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.create_texture(
            sdl2::pixels::PixelFormatEnum::RGB24,
            sdl2::render::TextureAccess::Static,
            viewport.cols, viewport.rows
        ).unwrap();
        texture.update(None, &self.draw_buffer, (viewport.cols * 3) as usize).unwrap();
        canvas.copy(&texture, None, Some(Rect::new(0, 0, viewport.cols, viewport.rows))).unwrap();
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