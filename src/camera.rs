use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec::Vec3;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;


#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: Vec3, // observador
    pub bg_color: Color,
    pub viewport: Viewport // janela
}

impl Camera {
    #[inline]
    pub fn new(pos: Vec3, n_rows: u32, n_cols: u32, viewport_h: f32, viewport_w: f32, viewport_distance: f32, bg_color: Color) -> Camera {
        Camera {
            pos: pos, // posição do observador
            bg_color: bg_color,
            viewport: Viewport::new(
                Vec3::new(pos.x, pos.y, pos.z-viewport_distance), // posição da janela em relação ao observador
                viewport_h, viewport_w, // altura * largura da janela
                n_rows, n_cols // número de colunas e linhas, basicamente a resolução da câmera.
            )
        }
    }

    #[inline]
    fn draw_pixel_on_canvas(self, canvas:&mut Canvas<Window>, px: i32, py: i32, color: Color) {
        canvas.set_draw_color(color);
        canvas.draw_point(Point::new(px,py)).unwrap();
    }

    pub fn draw_sphere_to_canvas(self, canvas: &mut Canvas<Window>, sphere: &Sphere) {
        canvas.set_draw_color(sphere.color);
        for row in 0..(self.viewport.rows as i32) {
            for col in 0..(self.viewport.cols as i32) {
                let direction: Vec3 =
                    (self.viewport.p00_coords
                    + (col as f32)*self.viewport.dx
                    - (row as f32)*self.viewport.dy)
                    - self.pos;
                let ray = Ray::new(self.pos, direction);

                // R(t) = p0 + t*direction

                let v: Vec3 = sphere.center - ray.origin;
                let a: f32 = ray.dir.dot(ray.dir);
                let b: f32 = (-2.0 * ray.dir).dot(v);
                let c: f32 = v.dot(v) - sphere.radius*sphere.radius;
                let delta: f32 = b*b - 4.0*a*c;

                if delta > 0.0 {
                    let t1 = (-b + delta.sqrt()) / (2.0*a);
                    let t2 = (-b - delta.sqrt()) / (2.0*a);
                    if t1 > 0.0 || t2 > 0.0 { // checa se pelo menos um t é positivo (a bola não está atrás do observador)
                        let min_t: f32 = t1.abs().min(t2.abs()); // pega o menor t positivo
                        let _collision_point = ray.at(min_t); // coordenada do ponto de interseção entre o raio e a esfera
                        self.draw_pixel_on_canvas(canvas, col, row, sphere.color); // se houve interseção, desenha com a cor da esfera
                    } else {
                        self.draw_pixel_on_canvas(canvas, col, row, self.bg_color);
                    }
                } else {
                    self.draw_pixel_on_canvas(canvas, col, row, self.bg_color); // se não houve interseção, desenha com a cor do background
                }
            }
        }
    }
}


#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Viewport {
    pub pos: Vec3,
    pub height: f32, pub width: f32,
    pub rows: u32, pub cols: u32,

    pub dy: Vec3, pub dx: Vec3, // tamanho x e y de cada quadrado
    pub top_left_coords: Vec3, // coordenadas da quina superior esquerda do frame
    pub p00_coords: Vec3 // coordenadas do quadrado 0,0 do frame
}

impl Viewport {
    #[inline]
    pub fn new(pos: Vec3, height: f32, width: f32, rows: u32, cols: u32) -> Viewport {
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