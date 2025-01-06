#ifndef SCENE_H
#define SCENE_H

#include <cmath>
#include <tuple>
#include <vector>
#include "shapes/shape.h"
#include "light.h"

class Scene {
    public:
        vector<Shape*> objects;
        vector<Light> lights;
        Vec3 ambient_light;
        
        Scene (): objects(vector<Shape*>()), lights(vector<Light>()), ambient_light(Vec3(1.0,1.0,1.0)) {}
        Scene (Vec3 ambient_light): objects(vector<Shape*>()), lights(vector<Light>()), ambient_light(ambient_light) {}
        
        void add_object(Shape* obj) { objects.push_back(obj); }
        void remove_object(int n) { objects.erase(objects.begin() + n); }
        bool remove_object(Shape* obj) { // retorna true se o objeto existia na cena, e false se não
            for (size_t i = 0; i < objects.size(); i++) {
                if (&objects.at(i) == &obj) { objects.erase(objects.begin() + i); return true; }
            };
            return false;
        }

        tuple<Shape*, double> get_closest_object(Ray ray) {
            double min_t = INFINITY;
            Shape* best_shape;
            
            // Pega o objeto mais próximo do raio (com t positivo, na frente do raio.)
            for ( Shape* s_candidate : this->objects ) {
                double t_candidate = s_candidate->intersects(ray);
                if (t_candidate >= 0.0 && t_candidate < min_t) {
                    min_t = t_candidate;
                    best_shape = s_candidate;
                }
            }

            if (min_t == INFINITY) { min_t = -INFINITY; } // O programa espera que NÃO COLISÃO = -INFINITO

            return make_tuple(best_shape, min_t);
        }

        void add_light(Light l) { lights.push_back(l); }
        void remove_light(int n) { lights.erase(lights.begin() + n); }
        bool remove_light(Light l) {
            for (size_t i = 0; i < lights.size(); i++) {
                if (&lights.at(i) == &l) { lights.erase(lights.begin() + i); return true; }
            };
            return false;
        }
};

#endif