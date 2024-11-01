use std::f32::INFINITY;
use super::Ray;
use super::Scene;
use super::shapes::Material;
use super::shapes::Shape;
use crate::utils::Vec3;
use crate::utils::vec_to_color;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;


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

    #[inline]
    // wrapper simples pra desenhar um pixel de cor <color> no ponto (px,py) de um canvas
    fn draw_pixel(&self, canvas:&mut Canvas<Window>, px: i32, py: i32, color: Color) {
        canvas.set_draw_color(color);
        canvas.draw_point(Point::new(px,py)).unwrap();
    }

    // draws entire scene
    pub fn draw_scene(&mut self, canvas: &mut Canvas<Window>, scene: &Scene) {
        canvas.set_draw_color(self.bg_color);
        canvas.clear();
        let mut ray = Ray::new(self.pos, Vec3::new(0.0,0.0,1.0)); // cria um raio partindo de p0 "atirado" na direção d
        let mut mat: &Material;
        
        for row in 0..(self.viewport.rows as i32) {
            for col in 0..(self.viewport.cols as i32) {
                let dr = ((self.viewport.p00_coords + (col as f32)*self.viewport.dx - (row as f32)*self.viewport.dy) - self.pos).normalize();
                ray.dr = dr;
                
                let mut shape: Option<&Shape> = None;
                let mut t = INFINITY;
                for s in &scene.shapes {
                    let t_s = s.intersects(&ray);
                    if t_s > 0.0 && t_s < t {
                        shape = Some(s);
                        t = t_s;
                    }
                }
                if shape.is_none() { continue; }
                let shape = shape.unwrap();
                mat = shape.material();
                let mut ieye = mat.k_amb * scene.ambient_light;
                let p_i = ray.at(t); // ponto de interseção 

                for light in &scene.lights {
                    let l = (light.pos - p_i).normalize(); // vetor apontando na direção da luz
                    let mut idif = Vec3::NULL; // cor vindo de reflexão difusa
                    let mut iesp = Vec3::NULL; // cor vindo de reflexão especular
                    let mut under_light = true;

                    let light_ray = Ray::new(p_i, light.pos - p_i);
                    for s in &scene.shapes {
                        if s == shape { continue; }
                        let tl = s.intersects(&light_ray);
                        if tl < 1.0 && tl > 0.0001 { under_light = false; break; }
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


                self.draw_pixel(canvas, col, row, vec_to_color(ieye.rgb_255()));
            }

        }
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