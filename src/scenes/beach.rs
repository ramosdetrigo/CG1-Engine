use std::f64::consts::PI;
use std::path::Path;

use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;

use crate::engine::{Scene, Light};
use crate::engine::camera::Camera;
use crate::utils::transform::{rotation_around_axis, scale_matrix, shear_matrix_x_angle, shear_matrix_y_angle, translation_matrix};
use crate::utils::Vec3;
use crate::engine::shapes::{Cilinder, Cone, Material, Mesh, Plane, Sphere, Texture};

pub fn beach() -> (Scene, Camera, u32, u32) {
    let p0 = Vec3::new(2.3, 1.3, 3.9); // posição do observador
    
    let aspect_ratio: f64 = 16.0/9.0; // aspect ratio que eu quero
    let image_width: u32 = 960; // Resolução da imagem (número de colunas e linhas na grade)
    let image_height: u32 = ((image_width as f64)/aspect_ratio) as u32;
    
    // let viewport_width: f64 = 0.032; // Tamanho da janela (em metros)
    // let viewport_height: f64 = viewport_width/aspect_ratio;
    let viewport_distance: f64 = 0.01; // distância da janela até o observador
    
    let bg_color = Vec3::new(0.35,0.63,0.95); // cor do background

    let sand_pc = Vec3::new(0.0, 0.0, 9.0); // Ponto conhecido do plano
    let sand_normal = Vec3::new(0.0, 1.0 , 0.0001); // Normal do plano
    let sand_material = Material::new(
        Vec3::new(0.92, 0.78, 0.65),
        Vec3::new(0.92, 0.78, 0.65),
        Vec3::all(0.5),
        3.0, 
    );

    let water_pc = Vec3::new(0.0, 0.0, 4.0); // Ponto conhecido do plano
    let water_normal = Vec3::new(0.0, 1.0 ,0.0); // Normal do plano
    let water_material = Material::new(
        Vec3::new(0.1, 0.6, 0.8),
        Vec3::new(0.1, 0.6, 0.8),
        Vec3::all(0.5),
        40.0, 
    );

    let snowball_material = Material::new(
        Vec3::all(0.9),
        Vec3::all(0.9),
        Vec3::all(0.3),
        3.0,
    );

    let snowman1_x = 3.0;
    let snowman1_base_y = -0.15;
    let snowman1_z = 7.0;
    let snowman1_feet_radius = 0.4;
    let snowman1_feet_center = Vec3::new(
        snowman1_x,
        snowman1_base_y + snowman1_feet_radius/1.5,
        snowman1_z
    );
    
    let snowman1_torso_radius = 0.325;
    let snowman1_torso_center = Vec3::new(
        snowman1_x,
        snowman1_feet_center.y + snowman1_feet_radius + snowman1_torso_radius/2.0,
        snowman1_z
    );

    let snowman1_head_radius = 0.25;
    let snowman1_head_center = Vec3::new(
        snowman1_x,
        snowman1_torso_center.y + snowman1_torso_radius + snowman1_head_radius/2.0,
        snowman1_z
    );

    let hat1_material = Material::new(
        Vec3::all(0.1),
        Vec3::all(0.1),
        Vec3::all(0.8),
        15.0,
    );
    let hat1_direction = Vec3::Y;

    let hat1_base_radius = 0.3;
    let hat1_base_height = 0.025;
    let hat1_base_cb = Vec3::new(
        snowman1_x,
        snowman1_head_center.y+snowman1_head_radius-hat1_base_height*2.0,
        snowman1_z
    );

    let hat1_body_radius = 0.225;
    let hat1_body_height = 0.4;
    let hat1_body_cb = Vec3::new(
        snowman1_x,
        hat1_base_cb.y+hat1_base_height,
        snowman1_z
    );

    let hat1_ribbon_material = Material::new(
        Vec3::new(0.6,0.05, 0.05),
        Vec3::new(0.6,0.05, 0.05),
        Vec3::new(0.6,0.05, 0.05),
        hat1_material.e,
    );
    let hat1_ribbon_radius = hat1_body_radius + 0.001;
    let hat1_ribbon_height = 0.05;
    let hat1_ribbon_cb = Vec3::new(
        snowman1_x,
        hat1_body_cb.y+0.05,
        snowman1_z
    );

    let nose_material = Material::new(
        Vec3::new(0.95,0.5, 0.15),
        Vec3::new(0.95,0.5, 0.15),
        Vec3::new(0.95,0.5, 0.15),
        3.0
    );
    let nose_radius = 0.05;
    let nose_height = 0.1;
    let nose_direction = -Vec3::Z;
    let nose_cb = Vec3::new(
        snowman1_x,
        snowman1_head_center.y,
        snowman1_z - snowman1_head_radius
    );

    let eye_material = Material::new(
        Vec3::all(0.1),
        Vec3::all(0.1),
        Vec3::all(0.3),
        3.0,
    );

    let snowman1_eye_radius = 0.05;
    let snowman1_eye_y = nose_cb.y + snowman1_eye_radius;
    let snowman1_eye_z = nose_cb.z + snowman1_eye_radius;

    let leye_center = Vec3::new(
        snowman1_x-nose_radius*2.0,
        snowman1_eye_y,
        snowman1_eye_z
    );
    
    let reye_center = Vec3::new(
        snowman1_x+nose_radius*2.0,
        snowman1_eye_y,
        snowman1_eye_z
    );

    let umbrella_direction = Vec3::new(0.2, 1.0, 0.0);
    let umbrella_top_height = 0.5;

    let umbrella_pole_material = Material::new(
        Vec3::all(0.5),
        Vec3::all(0.5),
        Vec3::all(0.2),
        20.0
    );
    let umbrella_pole_radius = 0.075;
    let umbrella_pole_height = 3.1;
    let umbrella_pole_cb = Vec3::new(snowman1_x + 2.0, -0.20, snowman1_z);

    let umbrella_top_material = Material::new(
        Vec3::new(0.9, 0.3, 0.3),
        Vec3::new(0.9, 0.3, 0.3),
        Vec3::new(0.9, 0.3, 0.3),
        3.0
    );
    let umbrella_top_radius = 2.0;
    let umbrella_top_cb = umbrella_pole_cb + umbrella_direction*(umbrella_pole_height-umbrella_top_height-0.037);


    let ball_radius = 0.6; // Raio em metros
    let ball_center = Vec3::new(7.0, ball_radius-0.1, 5.5); // Coords. centro da esfera (metros)
    let ball_material = Material::new(
        Vec3::new(0.9, 0.9, 0.9), // Ambient
        Vec3::new(0.9, 0.9, 0.9), // Diffuse
        Vec3::new(0.9, 0.9, 0.9), // Specular
        10.0, // coeficiente de "brilho" ou "polimento"
    );

    let ball_texture = Texture::new(
        RWops::from_file(Path::new("textures/beach_ball.png"), "r").unwrap().load_png().unwrap()
    );


    #[allow(unused_variables)]
    let cilinder_x = Cilinder::new(
        0.02, 200.0, 
        Vec3::NULL, Vec3::X, 
        Material::RED, 
        true, true
    );
    #[allow(unused_variables)]
    let cilinder_y = Cilinder::new(
        0.02, 200.0, 
        Vec3::NULL, Vec3::Y, 
        Material::GREEN, 
        true, true
    );
    #[allow(unused_variables)]
    let cilinder_z = Cilinder::new(
        0.02, 200.0, 
        Vec3::NULL, Vec3::Z, 
        Material::BLUE, 
        true, true
    );

    // Definindo as propriedades das luzes
    let light1_pos = Vec3::new(100.0, 200.0, 0.0);
    let light1_color = Vec3::new(1.0, 1.0, 1.0);
    let light1_intensity = 0.65;

    let chair_material = Material::new(
        Vec3::new(0.5, 0.3, 0.1), 
        Vec3::new(0.5, 0.3, 0.1), 
        Vec3::new(0.5, 0.3, 0.1), 
    5.0
    );
    
    let mut chair_middle = Mesh::cube(chair_material);
    let transform1 = translation_matrix(snowman1_x + 0.65, 0.4, snowman1_z - 1.2)
        * rotation_around_axis(Vec3::Y, -PI/2.0)
        * shear_matrix_y_angle(0.15)
        * rotation_around_axis(Vec3::Y, PI/2.0)
        * scale_matrix(0.75, 0.12, 1.2);
    chair_middle.apply_transform(&transform1);

    let max_y = chair_middle.vertices.iter().max_by(|vertex1, vertex2| {
        vertex1.y.partial_cmp(&vertex2.y).unwrap()
    }).unwrap().y;
    let mut chair_top = Mesh::cube(chair_material);
    let transform2 = translation_matrix(snowman1_x + 0.65, max_y-0.12, snowman1_z)
        * rotation_around_axis(Vec3::Y, -PI/2.0)
        * shear_matrix_y_angle(0.7)
        * rotation_around_axis(Vec3::Y, PI/2.0)
        * scale_matrix(0.75, 0.12, 0.75);
    chair_top.apply_transform(&transform2);

    let min_y = chair_middle.vertices.iter().min_by(|vertex1, vertex2| {
        vertex1.y.partial_cmp(&vertex2.y).unwrap()
    }).unwrap().y;
    let mut chair_bottom = Mesh::cube(chair_material);
    let transform2 = translation_matrix(snowman1_x + 0.65, min_y-0.81+0.18, snowman1_z - 1.95)
        * rotation_around_axis(Vec3::Y, -PI/2.0)
        * shear_matrix_y_angle(0.7)
        * rotation_around_axis(Vec3::Y, PI/2.0)
        * scale_matrix(0.75, 0.12, 0.75);
    chair_bottom.apply_transform(&transform2);

    
    // Criando os objetos e as luzes
    let shapes = vec![
        Plane::new( sand_pc, sand_normal, sand_material ), // chão
        Plane::new( water_pc, water_normal, water_material ), // fundo

        // bola de praia
        Sphere::new( ball_center, ball_radius, ball_material, Some(ball_texture) ),

        // snowman 1
        Sphere::new( snowman1_feet_center, snowman1_feet_radius, snowball_material, None ), // pé
        Sphere::new( snowman1_torso_center, snowman1_torso_radius, snowball_material, None ), // torso
        Sphere::new( snowman1_head_center, snowman1_head_radius, snowball_material, None ), // cabeça

        Cilinder::new( hat1_base_radius, hat1_base_height, hat1_base_cb, hat1_direction, hat1_material, true, true ), // hat base
        Cilinder::new( hat1_body_radius, hat1_body_height, hat1_body_cb, hat1_direction, hat1_material, true, true ), // hat body
        Cilinder::new( hat1_ribbon_radius, hat1_ribbon_height, hat1_ribbon_cb, hat1_direction, hat1_ribbon_material, true, true ), // hat ribbon

        Cone::new( nose_radius, nose_height, nose_cb, nose_direction, nose_material, true ), // nose
        
        Sphere::new( leye_center, snowman1_eye_radius, eye_material, None ), // left eye
        Sphere::new( reye_center, snowman1_eye_radius, eye_material, None ), // right eye

        Cilinder::new( umbrella_pole_radius, umbrella_pole_height, umbrella_pole_cb, umbrella_direction, umbrella_pole_material, true, true ),
        Cone::new( umbrella_top_radius, umbrella_top_height, umbrella_top_cb, umbrella_direction, umbrella_top_material, false ),

        chair_middle.into_shape(),
        chair_top.into_shape(),
        chair_bottom.into_shape(),

        // cilinder_x, cilinder_y, cilinder_z
    ];

    let lights = vec![
        Light::new( light1_pos, light1_color, light1_intensity ),
    ];

    let ambient_light = Vec3::new(0.3, 0.3, 0.3); // Luz ambiente
    let scene = Scene::new(shapes, lights, ambient_light);
    
    let mut camera: Camera = Camera::new(
        p0, // a posição do observador
        image_width, image_height, // número de colunas e linhas na grade (basicamente a resolução)
        90.0, // tamanho da janela (em metros)
        viewport_distance, // distância da janela até o observador (em metros)
        bg_color, // cor do background
    );

    camera.rotate(Vec3::Y, PI + PI/3.0);

    (scene, camera, image_width, image_height)
}