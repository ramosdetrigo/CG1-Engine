#![allow(dead_code)]
use super::Material;
use super::Shape;
use super::super::Ray;
use crate::utils::Matrix4;
use crate::utils::Vec3;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
/// Triângulo definido por três vértices `v0`, `v1`, `v2` e material `material`.
pub struct Triangle {
    pub v0: Vec3, // Primeiro vértice
    pub v1: Vec3, // Segundo vértice
    pub v2: Vec3, // Terceiro vértice
}

impl Triangle {
    #[inline]
    #[must_use]
    /// Cria um novo triângulo com os vértices `v0`, `v1`, `v2` e material `material`.
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Triangle {
        Self { v0, v1, v2 }
    }

    pub fn center(&self) -> Vec3 {
        (self.v0 + self.v1 + self.v2) / 3.0
    }
}

impl Triangle {
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um triângulo e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    fn intersects(&self, r: &Ray) -> f64 {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = r.dr.cross(edge2);
        let a = edge1.dot(h);

        if a > -1e-8 && a < 1e-8 {
            return f64::NEG_INFINITY; // O raio é paralelo ao triângulo
        }

        let f = 1.0 / a;
        let s = r.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return f64::NEG_INFINITY; // O ponto está fora do triângulo
        }

        let q = s.cross(edge1);
        let v = f * r.dr.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return f64::NEG_INFINITY; // O ponto está fora do triângulo
        }

        // Cálculo do t para encontrar o ponto de interseção
        let t = f * edge2.dot(q);
        if t > 1e-8 { // O raio intersecta o triângulo
            return t;
        } else {
            return f64::NEG_INFINITY; // O triângulo está atrás do raio
        }
    }
    fn normal(&self) -> Vec3 {
        (self.v1 - self.v0).cross(self.v2 - self.v0).normalize()
    }
}


#[derive(Clone, PartialEq)]
/// A mesh is a collection of triangles.
pub struct Mesh {
    triangles: Vec<Triangle>, // List of triangles
    material: Material,       // Material of the mesh
    center: Vec3,
    radius: f64
    // TODO: CENTER OF THE MESH
}

impl Mesh {
    #[inline]
    #[must_use]
    /// Creates a new mesh from a list of triangles and a material.
    pub fn new(triangles: Vec<Triangle>, material: Material) -> Mesh {
        let (center, radius) = Self::calculate_bounding_sphere(&triangles);
        Self { triangles, material, center, radius }
    }

    pub fn into_shape(self) -> Box<dyn Shape> {
        Box::new(self)
    }

    fn intersects_bounding_sphere(&self, r: &Ray) -> bool {
        let v: Vec3 = self.center - r.origin;
        let a: f64 = r.dr.length_squared();
        let b: f64 = r.dr.dot(v); // TODO: Explicar otimização
        let c: f64 = v.length_squared() - self.radius*self.radius;
        let delta: f64 = b*b - a*c;
        
        // se o delta é positivo, houve colisão
        if delta >= 0.0 {
            let t1 = (b + delta.sqrt()) / a;
            let t2 = (b - delta.sqrt()) / a;
            // true se a colisão foi na frente (t > 1.0), false otherwise
            if t2 < 0.0 || t1 < t2 { t1 > 0.0 }
            else { t2 > 0.0 }
        } else {
            false
        }
    }

    fn calculate_bounding_sphere(triangles: &Vec<Triangle>) -> (Vec3, f64) {
        // Calculate the center of the bounding sphere
        let mut center = Vec3::NULL;
        for triangle in triangles {
            center += triangle.center()
        }
        center = center / triangles.len() as f64;

        // Calculate the radius of the bounding sphere
        let mut sphere_radius: f64 = 0.0;
        for triangle in triangles {
            let d1 = (triangle.v0 - center).length();
            let d2 = (triangle.v1 - center).length();
            let d3 = (triangle.v2 - center).length();
            sphere_radius = sphere_radius.max(d1).max(d2).max(d3);
        }

        (center, sphere_radius)
    }

    pub fn cube(material: Material) -> Mesh {
        // vértices de um cubo 1x1x1
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 1.0, 0.0);
        let v4 = Vec3::new(1.0, 1.0, 0.0);
        let v5 = Vec3::new(0.0, 0.0, 1.0);
        let v6 = Vec3::new(1.0, 0.0, 1.0);
        let v7 = Vec3::new(0.0, 1.0, 1.0);
        let v8 = Vec3::new(1.0, 1.0, 1.0);

        let triangles = vec![
            // back
            Triangle::new(v3, v2, v1), Triangle::new(v2, v3, v4),
            // left
            Triangle::new(v7, v3, v1), Triangle::new(v7, v1, v5),
            // right
            Triangle::new(v4, v6, v2), Triangle::new(v4, v8, v6),
            // front
            Triangle::new(v5, v6, v7), Triangle::new(v8, v7, v6),
            // top
            Triangle::new(v7, v4, v3), Triangle::new(v7, v8, v4),
            // bottom
            Triangle::new(v1, v2, v6), Triangle::new(v1, v6, v5),
        ];

        Self::new(triangles, material)
    }

    pub fn apply_transform(&mut self, transformation_matrix: &Matrix4) {
        for triangle in &mut self.triangles {
            triangle.v0 = triangle.v0.transform(*transformation_matrix);
            triangle.v1 = triangle.v1.transform(*transformation_matrix);
            triangle.v2 = triangle.v2.transform(*transformation_matrix);
        }
        (self.center, self.radius) = Self::calculate_bounding_sphere(&self.triangles);
    }

    pub fn scale(&mut self, scaling_vector: Vec3) {
        for triangle in &mut self.triangles {
            triangle.v0 *= scaling_vector;
            triangle.v1 *= scaling_vector;
            triangle.v2 *= scaling_vector;
        }
        (self.center, self.radius) = Self::calculate_bounding_sphere(&self.triangles);
    }

    pub fn translate(&mut self, translation_vector: Vec3) {
        for triangle in &mut self.triangles {
            triangle.v0 += translation_vector;
            triangle.v1 += translation_vector;
            triangle.v2 += translation_vector;
        }
        self.center += translation_vector;
    }
}

impl Shape for Mesh {
    #[must_use]
    /// Finds the closest intersection between the mesh and a ray.
    /// Returns `(t, normal)` where `t` is the distance along the ray, and `normal` is the surface normal.
    /// If no intersection is found, returns `(f64::NEG_INFINITY, Vec3::NULL)`.
    fn intersects(&self, r: &Ray) -> (f64, Vec3) {
        // Check if the ray intersects with the bounding sphere
        if !self.intersects_bounding_sphere(r) {
            return (f64::NEG_INFINITY, Vec3::NULL);
        }

        let mut closest_t = f64::INFINITY;
        let mut closest_normal = Vec3::NULL;

        // Check intersections with all triangles in the mesh
        for triangle in &self.triangles {
            let normal = triangle.normal();
            if normal.dot(r.dr) > 0.0 { continue }
            let t = triangle.intersects(r);
            if t > 1e-8 && t < closest_t {
                closest_t = t;
                closest_normal = normal;
            }
        }

        if closest_t != f64::INFINITY {
            (closest_t, closest_normal)
        } else {
            (f64::NEG_INFINITY, Vec3::NULL)
        }
    }

    fn translate(&mut self, translation_vector: Vec3) {
        for triangle in &mut self.triangles {
            triangle.v0 += translation_vector;
            triangle.v1 += translation_vector;
            triangle.v2 += translation_vector;
        }
    }

    #[inline]
    #[must_use]
    /// Returns the material of the mesh.
    fn material(&self) -> &Material {
        &self.material
    }
}