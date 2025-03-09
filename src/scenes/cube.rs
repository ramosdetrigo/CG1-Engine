use std::collections::HashSet;
use std::f64::consts::PI;
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj, Vertex};

use crate::engine::{Scene, Light};
use crate::engine::camera::Camera;
use crate::utils::{Matrix3, Vec3};
use crate::utils::transform::{self, householder_reflection, translation_matrix};
use crate::engine::shapes::{Cilinder, Cone, Material, Plane, Sphere, Mesh};

pub fn cube<'a>() -> (Scene, Camera<'a>, u32, u32) {    
    // Planos
    let plane1_pc = Vec3::new(0.0, -0.5, 0.0); // Ponto conhecido do plano
    let plane1_normal = Vec3::new(0.0, 1.0 ,0.0); // Normal do plano
    let plane1_material = Material::new(
        Vec3::all(0.4),
        Vec3::all(0.4),
        Vec3::all(0.0),
        3.0, 
    );

    let mesh2_material = Material::new(
        Vec3::new(0.9, 0.7, 0.3),
        Vec3::new(0.9, 0.7, 0.3),
        Vec3::new(0.9, 0.7, 0.3),
        10.0, 
    );
    
    let cube_pos = Vec3::new(0.0, 1.0, -2.0);
    let mut cube = Mesh::cube(mesh2_material);
    cube.translate(cube_pos);
    
    // Definindo as propriedades das luzes
    let light1_pos = Vec3::new(0.0, 0.8, 0.0);
    let light1_color = Vec3::new(1.0, 1.0, 1.0);
    let light1_intensity = 1.0;

    let cube_pos = cube_pos + Vec3::Z;
    let cilinder_x1 = Cilinder::new(
        0.03, 2000.0, 
        cube_pos, Vec3::X, 
        Material::RED, 
        true, true
    );
    let cilinder_y1 = Cilinder::new(
        0.03, 2000.0, 
        cube_pos, Vec3::Y, 
        Material::GREEN, 
        true, true
    );
    let cilinder_z1 = Cilinder::new(
        0.03, 2000.0, 
        cube_pos, -Vec3::Z, 
        Material::BLUE, 
        true, true
    );

    let cilinder_x2 = Cilinder::new(
        0.03, 2000.0, 
        cube_pos + Vec3::Y, Vec3::X, 
        Material::RED, 
        true, true
    );
    let cilinder_y2 = Cilinder::new(
        0.03, 2000.0, 
        cube_pos - Vec3::Z, Vec3::Y, 
        Material::GREEN, 
        true, true
    );
    let cilinder_y3 = Cilinder::new(
        0.03, 2000.0, 
        cube_pos + Vec3::X, Vec3::Y, 
        Material::GREEN, 
        true, true
    );
    let cilinder_z2 = Cilinder::new(
        0.03, 2000.0, 
        cube_pos + Vec3::Y, -Vec3::Z, 
        Material::BLUE, 
        true, true
    );
    
    // Criando os objetos e as luzes
    let shapes = vec![
        Plane::new( plane1_pc, plane1_normal, plane1_material, None, 1.0, 1.0 ),
        cube.into_shape(),
        cilinder_x1,
        cilinder_y1,
        cilinder_z1,
        cilinder_x2,
        cilinder_y2,
        cilinder_z2,
        cilinder_y3,
    ];
    
    let lights = vec![
        Light::point( light1_pos, light1_color, light1_intensity ),
    ];
    
    let bg_color = Vec3::new(0.0,0.0,0.0); // cor do background
    let ambient_light = Vec3::new(0.3, 0.3, 0.3); // Luz ambiente
    let scene = Scene::new(shapes, lights, ambient_light, bg_color);

    let p0 = Vec3::new(-1.5, 1.0, 0.5); // posição do observador
    let aspect_ratio: f64 = 16.0/9.0; // aspect ratio que eu quero
    let image_width: u32 = 960; // Resolução da imagem (número de colunas e linhas na grade)
    let image_height: u32 = ((image_width as f64)/aspect_ratio) as u32;
    let viewport_distance: f64 = 0.5; // distância da janela até o observador
    
    let mut camera: Camera = Camera::new(
        p0, // a posição do observador
        image_width, image_height, // número de colunas e linhas na grade (basicamente a resolução)
        3.2, 1.8, // tamanho da janela (em metros)
        viewport_distance, // distância da janela até o observador (em metros)
    );

    camera.look_at(cube_pos, Vec3::Y);

    println!("returning scene...");
    (scene, camera, image_width, image_height)
}   