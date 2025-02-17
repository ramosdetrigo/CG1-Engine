use std::collections::HashSet;
use std::sync::Arc;

use crate::engine::{Camera, Scene, Light};
use crate::utils::Vec3;
use crate::engine::shapes::{Cilinder, Cone, Material, Plane, Sphere, Triangle, Mesh};

pub fn simple() -> (Scene, Camera, u32, u32) {
    let p0 = Vec3::new(0.75, 1.4, 0.2); // posição do observador
    
    let aspect_ratio: f64 = 16.0/9.0; // aspect ratio que eu quero
    
    let image_width: u32 = 1920; // Resolução da imagem (número de colunas e linhas na grade)
    let image_height: u32 = ((image_width as f64)/aspect_ratio) as u32;
    
    let viewport_width: f64 = 0.032; // Tamanho da janela (em metros)
    let viewport_height: f64 = viewport_width/aspect_ratio;
    let viewport_distance: f64 = 0.01; // distância da janela até o observador
    
    let bg_color = Vec3::new(0.0,0.0,0.0); // cor do background
    
    // Planos
    let plane1_pc = Vec3::new(0.0, -0.5, 0.0); // Ponto conhecido do plano
    let plane1_normal = Vec3::new(0.0, 1.0 ,0.0); // Normal do plano
    let plane1_material = Material::new(
        Vec3::all(0.4),
        Vec3::all(0.4),
        Vec3::all(0.0),
        3.0, 
    );
    let plane2_pc = Vec3::new(0.0, 0.0, -5.0); // Ponto conhecido do plano
    let plane2_normal = Vec3::new(0.0, 0.0 ,1.0); // Normal do plano
    let plane2_material = Material::new(
        Vec3::new(0.4, 0.4, 0.7),
        Vec3::new(0.4, 0.4, 0.7),
        Vec3::all(0.0),
        3.0, 
    );

    // Definindo as propriedades de cada objeto
    let mesh1_material = Material::new(
        Vec3::new(0.3, 0.7, 0.3),
        Vec3::new(0.3, 0.7, 0.3),
        Vec3::new(0.3, 0.7, 0.3),
        10.0, 
    );
    
    // vértices de um cubo 1x1x1
    let v1 = Vec3::new(-0.5, 0.0, -2.0);
    let v2 = Vec3::new(0.5, 0.0, -2.0);
    let v3 = Vec3::new(-0.5, 1.0, -2.0);
    let v4 = Vec3::new(0.5, 1.0, -2.0);
    let v5 = Vec3::new(-0.5, 0.0, -1.0);
    let v6 = Vec3::new(0.5, 0.0, -1.0);
    let v7 = Vec3::new(-0.5, 1.0, -1.0);
    let v8 = Vec3::new(0.5, 1.0, -1.0);

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
    
    // Definindo as propriedades das luzes
    let light1_pos = Vec3::new(0.0, 0.8, 0.0);
    let light1_color = Vec3::new(1.0, 1.0, 1.0);
    let light1_intensity = 1.0;
    
    // Criando os objetos e as luzes
    let shapes = vec![
        Plane::new( plane1_pc, plane1_normal, plane1_material ),
        Plane::new( plane2_pc, plane2_normal, plane2_material ),
        Mesh::new(triangles, mesh1_material),
    ];
    
    let lights = vec![
        Light::new( light1_pos, light1_color, light1_intensity ),
    ];
    
    let ambient_light = Vec3::new(0.3, 0.3, 0.3); // Luz ambiente
    let scene = Scene::new(shapes, lights, ambient_light);
    
    let camera: Camera = Camera::new(
        p0, // a posição do observador
        image_width, image_height, // número de colunas e linhas na grade (basicamente a resolução)
        viewport_width, viewport_height, // tamanho da janela (em metros)
        viewport_distance, // distância da janela até o observador (em metros)
        bg_color, // cor do background
    );

    (scene, camera, image_width, image_height)
}   