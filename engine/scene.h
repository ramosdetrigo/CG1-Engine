#ifndef SCENE_H
#define SCENE_H

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