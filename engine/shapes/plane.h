#ifndef PLANE_H
#define PLANE_H

#include "shape.h"

class Plane : public Shape {
    public:
        Vec3 p0;
        Vec3 normal;
        Plane (): p0(Vec3(0.0, 0.0, 0.0)), normal(Vec3(0.0, -1.0, 0.0)), Shape() {}
        Plane (Vec3 p0, Vec3 normal, Material mat): p0(p0), normal(normal), Shape(mat) {}
        
        inline Vec3 get_normal(Vec3 p) override { return normal; }
        tuple<bool, float, float> intersects(Ray r) override {
            float top = normal.dot(r.origin - p0);
            float bottom = normal.dot(r.dr);
            if (bottom == 0.0) { return make_tuple(false, -1.0, -INFINITY); }
            return make_tuple(true, - top/bottom, -INFINITY);
        }
};

#endif