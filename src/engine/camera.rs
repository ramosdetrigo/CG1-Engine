use super::ray::Ray;
use super::sphere::Sphere;
use crate::utils::vec::Vec3;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;


#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: Vec3, // observador
    pub bg_color: Color,
    viewport: Viewport // janela
}

impl Camera {
    #[inline]
    pub fn new(pos: Vec3, n_cols: u32, n_rows: u32, viewport_w: f32, viewport_h: f32, viewport_distance: f32, bg_color: Color) -> Camera {
        Camera {
            pos: pos, // posição do observador
            bg_color: bg_color,
            viewport: Viewport::new(
                Vec3::new(pos.x, pos.y, pos.z-viewport_distance), // posição da janela em relação ao observador (0, 0, -d)
                viewport_w, viewport_h, // altura * largura da janela
                n_cols, n_rows // número de colunas e linhas, basicamente a resolução da câmera.
            )
        }
    }

    #[inline]
    // wrapper simples pra desenhar um pixel de cor <color> no ponto (px,py) de um canvas
    fn draw_pixel(self, canvas:&mut Canvas<Window>, px: i32, py: i32, color: Color) {
        canvas.set_draw_color(color);
        canvas.draw_point(Point::new(px,py)).unwrap();
    }

    pub fn draw_sphere(&self, canvas: &mut Canvas<Window>, sphere: &Sphere) {
        for row in 0..(self.viewport.rows as i32) { // linhas (eixo y)
            for col in 0..(self.viewport.cols as i32) { // colunas (eixo x)
                let direction: Vec3 = // direção do raio
                    (self.viewport.p00_coords
                    + (col as f32)*self.viewport.dx
                    - (row as f32)*self.viewport.dy)
                    - self.pos;
                
                let ray = Ray::new(self.pos, direction); // cria um raio partindo de p0 "atirado" na direção d
                let (intersect, t1, t2) = ray.intersects_sphere(sphere); // checa se o raio intersecta a esfera
                
                if intersect && (t1 > 0.0 || t2 > 0.0) {
                    let min_t = if t2 <= 0.0 || t1 < t2 {t1} else {t2}; // obtém o menor t positivo
                    let _collision_point = ray.at(min_t);
                    // bola na frente do observador, pinta a cor da esfera
                    self.draw_pixel(canvas, col, row, sphere.color);
                } else {
                    // não houve interseção ou a bola está atrás do observador, pinta a cor do background
                    self.draw_pixel(canvas, col, row, self.bg_color);
                }
            }
        }
    }
}


#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
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
            top_left_coords, p00_coords
        }
    }
}