#ifndef CAMERA_H
#define CAMERA_H

#include <SDL_render.h>
#include "scene.h"
#include "ray.h"
#include "light.h"
#include "shapes/shape.h"

class Camera {
    public:
        Vec3 pos, bg_color;
        Camera () : pos(Vec3()), bg_color(Vec3(1.0, 1.0, 1.0)), viewport(Viewport()) {}
        Camera (Vec3 pos, double width, double height, double cols, double rows, double viewport_distance, Vec3 bg_color) :
            pos(pos), bg_color(bg_color), viewport(Viewport(Vec3(pos.x, pos.y, pos.z - viewport_distance), width, height, cols, rows)) {}

        void draw_scene(SDL_Renderer* renderer, Scene scene) {
            SDL_SetRenderDrawColor(renderer, bg_color.x, bg_color.y, bg_color.z, 1.0);
            SDL_RenderClear(renderer);
            Light light = scene.lights.front(); // pegando só a primeira luz por enquanto...              

            for (int row = 0; row < viewport.rows; row++) { // cada linha
                for (int col = 0; col < viewport.cols; col++ ) { // cada coluna
                    // vetor direção pro quadrado do frame
                    Vec3 dr = ((viewport.p00 + viewport.dx * col - viewport.dy * row) - pos).normalize();
                    Ray r = Ray(pos, dr); // nosso raio

                    // pega o objeto mais próximo na cena
                    auto [closest_shape, t] = scene.get_closest_object(r);

                    // se ele não estiver atrás da câmera, calcula aS luzes (mas testa pra sombra antes né etc.)
                    if (t > 0.0) {
                        Vec3 p_intersect = r.at(t); // Ponto de interseção do raio com o objeto
                        Vec3 ieye = Vec3(0.0, 0.0, 0.0);
                        
                        // Testa todas as luzes da cena
                        for (Light light : scene.lights) {
                            // Check da sombra
                            bool na_sombra = false;
                            // Raio do ponto de interseção até a luz (não normaliza o vetor direção)
                            Ray raio_p_luz = Ray(p_intersect, light.pos - p_intersect);
                            // testa pra todos os objetos da cena pra ver se eles tão na frente da luz.
                            for (Shape* s_test : scene.objects) {
                                // distância do ponto de interseção até o ponto de luz
                                double distance = s_test->intersects(raio_p_luz);

                                // 0.0001 evita problemas de precisão double
                                // isso checa se o objeto está ENTRE a interseção e o raio de luz
                                if (distance >= 0.0001 && distance <= 1.0) {
                                    na_sombra = true; // se sim, o objeto q a gente ia desenhar tá na sombra.
                                    break;
                                }
                            }

                            Vec3 l = (light.pos - p_intersect).normalize(); // Vetor apontando na direção da luz
                            Vec3 n = closest_shape->get_normal(p_intersect); // Vetor normal
                            Vec3 r = (2.0 * (l.dot(n)))*n - l; // Vetor l refletido na normal
                            Vec3 v = -dr; // Vetor apontando na direção do observador

                            double nl = n.dot(l); // N escalar L
                            double rv = r.dot(v); // R escalar V
                            if (nl < 0.0 || na_sombra) { nl = 0.0; }
                            if (rv < 0.0 || na_sombra) { rv = 0.0; }

                            Vec3 iamb = closest_shape->mat.k_ambient * scene.ambient_light;
                            Vec3 idif = closest_shape->mat.k_diffuse * nl * light.color;
                            Vec3 iesp = closest_shape->mat.k_specular * pow(rv, closest_shape->mat.e) * light.color;

                            ieye = ieye + iamb + idif + iesp;
                        }

                        draw_pixel(renderer, col, row, ieye.clamp(0.0, 1.0).rgb_255());
                    }
                }
            }

            SDL_RenderPresent(renderer);
        }
    
    private:
        inline void draw_pixel(SDL_Renderer* renderer, int x, int y, Vec3 color) {
            SDL_SetRenderDrawColor(renderer, color.x, color.y, color.z, 1.0);
            SDL_RenderDrawPoint(renderer, x, y);
        }

        class Viewport {
        public:
            Vec3 pos, dx, dy, top_left, p00;
            double width, height;
            int cols, rows;
            
            Viewport () {
                Vec3 pos = Vec3(0.0, 0.0, -1.0);
                double width = 1.0; double height = 1.0;
                double cols = 256; double rows = 256;

                Vec3 dx = Vec3(width/cols, 0.0, 0.0);
                Vec3 dy = Vec3(0.0, height/cols, 0.0);
                Vec3 top_left = Vec3(pos.x - width/2.0, pos.y + height/2.0, pos.z);
                Vec3 p00 = top_left + dx/2 - dy/2;

                this->pos = pos; this->dx = dx; this->dy = dy; this->top_left = top_left; this->p00 = p00;
                this->width = width; this->height = height;
                this->cols = cols; this->rows = rows;
            }

            Viewport (Vec3 pos, double width, double height, double cols, double rows) {
                Vec3 dx = Vec3(width/cols, 0.0, 0.0);
                Vec3 dy = Vec3(0.0, height/rows, 0.0);
                Vec3 top_left = Vec3(pos.x - width/2.0, pos.y + height/2.0, pos.z);
                Vec3 p00 = top_left + dx/2 - dy/2;

                this->pos = pos; this->dx = dx; this->dy = dy; this->top_left = top_left; this->p00 = p00;
                this->width = width; this->height = height;
                this->cols = cols; this->rows = rows;      
            }
        };
        Viewport viewport;
};

#endif