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
}

impl Triangle {
    #[must_use]
    /// Retorna o ponto de interseção (de distância positiva) mais próximo entre um triângulo e um raio `r` \
    /// (`-INFINITY` se não há interseção)
    fn intersects(&self, r: &Ray) -> (f64, Vec3) {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = r.dr.cross(edge2);
        let a = edge1.dot(h);

        if a > -1e-8 && a < 1e-8 {
            return (f64::NEG_INFINITY, Vec3::NULL); // O raio é paralelo ao triângulo
        }

        let f = 1.0 / a;
        let s = r.origin - self.v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return (f64::NEG_INFINITY, Vec3::NULL); // O ponto está fora do triângulo
        }

        let q = s.cross(edge1);
        let v = f * r.dr.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return (f64::NEG_INFINITY, Vec3::NULL); // O ponto está fora do triângulo
        }

        // Cálculo do t para encontrar o ponto de interseção
        let t = f * edge2.dot(q);
        if t > 1e-8 { // O raio intersecta o triângulo
            let n = edge1.cross(edge2).normalize(); // Normal do triângulo
            return (t, n);
        } else {
            return (f64::NEG_INFINITY, Vec3::NULL); // O triângulo está atrás do raio
        }
    }
}


#[derive(Clone, PartialEq)]
/// A mesh is a collection of triangles.
pub struct Mesh {
    triangles: Vec<Triangle>, // List of triangles
    material: Material,       // Material of the mesh
    // TODO: CENTER OF THE MESH
}

impl Mesh {
    #[inline]
    #[must_use]
    /// Creates a new mesh from a list of triangles and a material.
    pub fn new(triangles: Vec<Triangle>, material: Material) -> Mesh {
        Self { triangles, material }
    }

    pub fn into_shape(self) -> Box<dyn Shape> {
        Box::new(self)
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

    pub fn apply_transform(&mut self, transformation_matrix: Matrix4) {
        for triangle in &mut self.triangles {
            triangle.v0 = triangle.v0.transform(transformation_matrix);
            triangle.v1 = triangle.v1.transform(transformation_matrix);
            triangle.v2 = triangle.v2.transform(transformation_matrix);
        }
    }

    pub fn scale(&mut self, scaling_vector: Vec3) {
        for triangle in &mut self.triangles {
            triangle.v0 *= scaling_vector;
            triangle.v1 *= scaling_vector;
            triangle.v2 *= scaling_vector;
        }
    }

    pub fn translate(&mut self, translation_vector: Vec3) {
        for triangle in &mut self.triangles {
            triangle.v0 += translation_vector;
            triangle.v1 += translation_vector;
            triangle.v2 += translation_vector;
        }
    }
}

impl Shape for Mesh {
    #[must_use]
    /// Finds the closest intersection between the mesh and a ray.
    /// Returns `(t, normal)` where `t` is the distance along the ray, and `normal` is the surface normal.
    /// If no intersection is found, returns `(f64::NEG_INFINITY, Vec3::NULL)`.
    fn intersects(&self, r: &Ray) -> (f64, Vec3) {
        let mut closest_t = f64::INFINITY;
        let mut closest_normal = Vec3::NULL;

        // Check intersections with all triangles in the mesh
        for triangle in &self.triangles {
            let (t, normal) = triangle.intersects(r);
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