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
/// A mesh is a collection of triangles.
pub struct Mesh {
    vertices: Vec<Vec3>,
    triangles: Vec<[usize; 3]>, // List of triangles
    material: Material,       // Material of the mesh
    // center: Vec3,
    min_bound: Vec3,
    max_bound: Vec3,
}

impl Mesh {
    #[inline]
    #[must_use]
    /// Creates a new mesh from a list of triangles and a material.
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<[usize; 3]>, material: Material) -> Mesh {
        let (min_bound, max_bound) = Self::calculate_bounding_box(&vertices);
        Self { vertices, triangles, material, min_bound, max_bound }
    }

    pub fn into_shape(self) -> Box<dyn Shape> {
        Box::new(self)
    }

    fn intersects_bounding_box(&self, r: &Ray) -> bool {
        let t_min = (self.min_bound.x - r.origin.x) / r.dr.x;
        let t_max = (self.max_bound.x - r.origin.x) / r.dr.x;
        let (t_min_x, t_max_x) = if r.dr.x < 0.0 { (t_max, t_min) } else { (t_min, t_max) };

        let t_min_y = (self.min_bound.y - r.origin.y) / r.dr.y;
        let t_max_y = (self.max_bound.y - r.origin.y) / r.dr.y;
        let (t_min_y, t_max_y) = if r.dr.y < 0.0 { (t_max_y, t_min_y) } else { (t_min_y, t_max_y) };

        let t_min_z = (self.min_bound.z - r.origin.z) / r.dr.z;
        let t_max_z = (self.max_bound.z - r.origin.z) / r.dr.z;
        let (t_min_z, t_max_z) = if r.dr.z < 0.0 { (t_max_z, t_min_z) } else { (t_min_z, t_max_z) };

        if t_max_x < t_min_y || t_max_y < t_min_x {
            return false;
        }
        let t_min = t_min_x.max(t_min_y).max(t_min_z);
        let t_max = t_max_x.min(t_max_y).min(t_max_z);

        t_min <= t_max && t_max >= 0.0
    }

    fn calculate_bounding_box(vertices: &Vec<Vec3>) -> (Vec3, Vec3) {
        // Calculate the minimum and maximum bounds of the bounding box
        let mut min_bound = vertices[0].clone();
        let mut max_bound = vertices[0].clone();

        for vertex in vertices {
            min_bound.x = min_bound.x.min(vertex.x);
            min_bound.y = min_bound.y.min(vertex.y);
            min_bound.z = min_bound.z.min(vertex.z);

            max_bound.x = max_bound.x.max(vertex.x);
            max_bound.y = max_bound.y.max(vertex.y);
            max_bound.z = max_bound.z.max(vertex.z);
        }

        (min_bound, max_bound)
    }

    pub fn cube(material: Material) -> Mesh {
        // vértices de um cubo 1x1x1
        let vertices: Vec<Vec3> = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
        ];

        let triangles: Vec<[usize; 3]> = vec![
            // back
            [2, 1, 0], [1, 2, 3],
            // left
            [6, 2, 0], [6, 0, 4],
            // right
            [3, 5, 1], [3, 7, 5],
            // front
            [4, 5, 6], [7, 6, 5],
            // top
            [6, 3, 2], [6, 7, 3],
            // bottom
            [0, 1, 5], [0, 5, 4],
        ];

        Self::new(vertices, triangles, material)
    }

    pub fn apply_transform(&mut self, transformation_matrix: &Matrix4) {
        for vertex in &mut self.vertices {
            *vertex = vertex.transform(*transformation_matrix);
        }
        (self.min_bound, self.max_bound) = Self::calculate_bounding_box(&self.vertices);
    }

    pub fn scale(&mut self, scaling_vector: Vec3) {
        for vertex in &mut self.vertices {
            *vertex *= scaling_vector;
        }
        (self.min_bound, self.max_bound) = Self::calculate_bounding_box(&self.vertices);
    }

    pub fn translate(&mut self, translation_vector: Vec3) {
        for vertex in &mut self.vertices {
            *vertex += translation_vector;
        }
        self.min_bound += translation_vector;
        self.max_bound += translation_vector;
    }

    fn triangle_normal(&self, triangle: &[usize;3]) -> Vec3 {
        (self.vertices[triangle[1]] - self.vertices[triangle[0]]).cross(self.vertices[triangle[2]] - self.vertices[triangle[0]]).normalize()
    }
    
    fn triangle_intersects(&self, triangle: &[usize;3], r: &Ray) -> f64 {
        let v0 = self.vertices[triangle[0]];
        let v1 = self.vertices[triangle[1]];
        let v2 = self.vertices[triangle[2]];

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = r.dr.cross(edge2);
        let a = edge1.dot(h);

        if a > -1e-8 && a < 1e-8 {
            return f64::NEG_INFINITY; // O raio é paralelo ao triângulo
        }

        let f = 1.0 / a;
        let s = r.origin - v0;
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
}

impl Shape for Mesh {
    #[must_use]
    /// Finds the closest intersection between the mesh and a ray.
    /// Returns `(t, normal)` where `t` is the distance along the ray, and `normal` is the surface normal.
    /// If no intersection is found, returns `(f64::NEG_INFINITY, Vec3::NULL)`.
    fn get_intersection(&self, r: &Ray) -> Option<(f64, Vec3)> {
        // Check if the ray intersects with the bounding sphere
        if !self.intersects_bounding_box(r) {
            return None;
        }

        self.triangles.iter()
            .filter_map(|triangle| {
                let normal = self.triangle_normal(triangle);
                if normal.dot(r.dr) >= 0.0 { return None }
                let t = self.triangle_intersects(triangle, r);
                (t > 1e-8).then_some((t, normal))
            })
            .min_by(|(t1,_), (t2,_)| t1.partial_cmp(t2).unwrap())
    }

    fn translate(&mut self, translation_vector: Vec3) {
        for vertex in &mut self.vertices {
            *vertex += translation_vector;
        }
        self.min_bound += translation_vector;
        self.max_bound += translation_vector;
    }

    #[inline]
    #[must_use]
    /// Returns the material of the mesh.
    fn material(&self) -> &Material {
        &self.material
    }
}