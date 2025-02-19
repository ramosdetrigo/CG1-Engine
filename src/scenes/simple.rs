use std::collections::HashSet;
use std::f64::consts::PI;
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj, Vertex};

use crate::engine::{Camera, Scene, Light};
use crate::utils::Vec3;
use crate::utils::transform::{self, translation_matrix};
use crate::engine::shapes::{Cilinder, Cone, Material, Plane, Sphere, Mesh};

pub fn simple() -> (Scene, Camera, u32, u32) {
    let p0 = Vec3::new(0.0, 0.4, 0.2); // posição do observador
    
    let aspect_ratio: f64 = 16.0/9.0; // aspect ratio que eu quero
    
    let image_width: u32 = 960; // Resolução da imagem (número de colunas e linhas na grade)
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

    let mesh2_material = Material::new(
        Vec3::new(0.9, 0.7, 0.3),
        Vec3::new(0.9, 0.7, 0.3),
        Vec3::new(0.9, 0.7, 0.3),
        10.0, 
    );
    
    let mut cube = Mesh::cube(mesh1_material);
    let trans_matrix1 = transform::translation_matrix(-0.5, 0.0, -2.0) // mover ele pro lugar q eu quero
        // * transform::translation_matrix(0.5, 0.5, 0.5) // desfazer a translação
        // * transform::rotation_around_axis(Vec3::X, PI*0.12) // girar ao redor do eixo X
        // * transform::rotation_around_axis(Vec3::Y, -PI*0.125) // girar ao redor do eixo Y
        // * transform::translation_matrix(-0.5, -0.5, -0.5) // centralizar o cubo no 0,0
        * transform::shear_matrix_y_angle(0.2) // shear é mó paia
        * transform::scale_matrix(1.0, 0.1, 1.0) // amassa o cubo (scale no eixo Y)
        ;
    cube.apply_transform(&trans_matrix1);

    let pyramid_vertices = vec![
        Vec3::new(0.0, 0.0, 0.0), // 0
        Vec3::new(1.0, 0.0, 0.0), // 1
        Vec3::new(1.0, 0.0, 1.0), // 2
        Vec3::new(0.0, 0.0, 1.0), // 3
        Vec3::new(0.5, 3.0_f64.sqrt() / 2.0, 0.5), // 4
    ];
    let pyramid_triangles = vec![
        [1, 0, 2], [2, 0, 3], // baixo
        [2, 4, 3], // FRENTE
        [1, 0, 4], // ATRÁS
        [0, 3, 4], // ESQUERDA
        [2, 1, 4], // DIREITA
    ];
    let mut pyramid = Mesh::new(pyramid_vertices, pyramid_triangles, mesh2_material);
    let trans_matrix2 = transform::translation_matrix(0.5, 0.0, -2.0) // mover ele pro lugar q eu quero
        * transform::translation_matrix(0.5, 0.5, 0.5) // desfazer a translação
        // * transform::rotation_around_axis(Vec3::X, PI*0.12) // girar ao redor do eixo X
        * transform::rotation_around_axis(Vec3::Y, PI*0.125) // girar ao redor do eixo Y
        * transform::translation_matrix(-0.5, -0.5, -0.5) // centralizar o cubo no 0,0
        // * transform::shear_matrix_y_angle(0.7) // shear é mó paia
        // * transform::scale_matrix(1.0, 0.1, 1.0); // amassa o cubo (scale no eixo Y)
        ;
    pyramid.apply_transform(&trans_matrix2);

    println!("starting obj importing...");
    let input = BufReader::new(File::open("teapot400.obj").unwrap());
    let model: Obj = obj::load_obj(input).unwrap();

    // model.vertices;
    println!("converting obj...");
    
    let teapot_vertices: Vec<Vec3> = model.vertices
        .into_iter()
        .map(|vertex| Vec3::new(vertex.position[0] as f64, vertex.position[1] as f64, vertex.position[2] as f64))
        .collect();

    let teapot_triangles: Vec<[usize; 3]> = model.indices
        .chunks(3)
        .map(|face| [face[0] as usize, face[1] as usize, face[2] as usize])
        .collect();


    println!("imported {:} triangles!", teapot_triangles.len());
    let mut teapot = Mesh::new(teapot_vertices, teapot_triangles, Material::WHITE);
    let teapot_trans = transform::rotation_around_axis(Vec3::Y, PI*0.5); // girar ao redor do eixo Y
    teapot.apply_transform(&teapot_trans);
    teapot.scale(Vec3::all(0.25));
    teapot.translate(Vec3::new(0.0, 0.0, -2.0));
    println!("finished scaling!!");
    
    // Definindo as propriedades das luzes
    let light1_pos = Vec3::new(0.0, 0.8, 0.0);
    let light1_color = Vec3::new(1.0, 1.0, 1.0);
    let light1_intensity = 1.0;
    
    // Criando os objetos e as luzes
    let shapes = vec![
        Plane::new( plane1_pc, plane1_normal, plane1_material ),
        Plane::new( plane2_pc, plane2_normal, plane2_material ),
        // cube.into_shape(),
        // pyramid.into_shape(),
        teapot.into_shape()
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

    println!("returning scene...");
    (scene, camera, image_width, image_height)
}   