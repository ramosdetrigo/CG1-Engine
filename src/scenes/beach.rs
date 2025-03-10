use std::f64::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use obj::Obj;
use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;

use crate::engine::{Scene, Light};
use crate::engine::camera::Camera;
use crate::utils::transform::{rotation_around_axis, scale_matrix, shear_matrix_y, translation_matrix};
use crate::utils::Vec3;
use crate::engine::shapes::{Cilinder, Cone, Material, Mesh, Plane, Sphere, Texture};

pub fn beach<'a>() -> (Scene, Camera<'a>, u32, u32) {    
    let sand_pc = Vec3::new(0.0, 0.0, 9.0); // Ponto conhecido do plano
    let sand_normal = Vec3::new(0.0, 1.0 , 0.0001); // Normal do plano
    // let sand_material = Material::new(
    //     Vec3::new(0.92, 0.78, 0.65),
    //     Vec3::new(0.92, 0.78, 0.65),
    //     Vec3::all(0.5),
    //     3.0, 
    // );
    let sand_material = Material::new(
        Vec3::new(0.9, 0.9, 0.9), // Ambient
        Vec3::new(0.9, 0.9, 0.9), // Diffuse
        Vec3::new(0.9, 0.9, 0.9), // Specular
        5.0, // coeficiente de "brilho" ou "polimento"
    );
    
    let water_pc = Vec3::new(0.0, 0.0, 4.0); // Ponto conhecido do plano
    let water_normal = Vec3::new(0.0, 1.0 ,0.0); // Normal do plano
    // let water_material = Material::new(
    //     Vec3::new(0.1, 0.6, 0.8),
    //     Vec3::new(0.1, 0.6, 0.8),
    //     Vec3::all(0.5),
    //     40.0, 
    // );
    let water_material = Material::new(
        Vec3::new(0.9, 0.9, 0.9), // Ambient
        Vec3::new(0.9, 0.9, 0.9), // Diffuse
        Vec3::new(0.9, 0.9, 0.9), // Specular
        50.0, // coeficiente de "brilho" ou "polimento"
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
        snowman1_head_center.y+snowman1_head_radius-hat1_base_height*3.0,
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
        50.0
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

    let chair_material = Material::new(
        Vec3::new(0.5, 0.3, 0.1), 
        Vec3::new(0.5, 0.3, 0.1), 
        Vec3::new(0.5, 0.3, 0.1), 
    5.0
    );
    
    let mut chair_middle = Mesh::cube(chair_material);
    let transform1 = translation_matrix(snowman1_x + 0.65, 0.4, snowman1_z - 1.2)
        * rotation_around_axis(Vec3::Y, -PI/2.0, Vec3::NULL)
        * shear_matrix_y(0.15, 0.0)
        * rotation_around_axis(Vec3::Y, PI/2.0, Vec3::NULL)
        * scale_matrix(0.75, 0.12, 1.2, Vec3::NULL);
    chair_middle.apply_transform(&transform1);

    let max_y = chair_middle.vertices.iter().max_by(|vertex1, vertex2| {
        vertex1.y.total_cmp(&vertex2.y)
    }).unwrap().y;
    let mut chair_top = Mesh::cube(chair_material);
    let transform2 = translation_matrix(snowman1_x + 0.65, max_y-0.12, snowman1_z)
        * rotation_around_axis(Vec3::Y, -PI/2.0,Vec3::NULL)
        * shear_matrix_y(0.7, 0.0)
        * rotation_around_axis(Vec3::Y, PI/2.0, Vec3::NULL)
        * scale_matrix(0.75, 0.12, 0.75, Vec3::NULL);
    chair_top.apply_transform(&transform2);

    let max_y = chair_middle.vertices.iter().max_by(|vertex1, vertex2| {
        vertex1.y.total_cmp(&vertex2.y)
    }).unwrap().y;
    let mut chair_leg_back = Mesh::cube(chair_material);
    let transform3 = translation_matrix(snowman1_x + 0.65, max_y-0.12, snowman1_z)
        * rotation_around_axis(Vec3::Y, -PI/2.0, Vec3::NULL)
        * shear_matrix_y(-0.7, 0.0)
        * rotation_around_axis(Vec3::Y, PI/2.0, Vec3::NULL)
        * scale_matrix(0.75, 0.12, 0.75, Vec3::NULL);
    chair_leg_back.apply_transform(&transform3);

    let min_y = chair_middle.vertices.iter().min_by(|vertex1, vertex2| {
        vertex1.y.total_cmp(&vertex2.y)
    }).unwrap().y;
    let mut chair_bottom = Mesh::cube(chair_material);
    let transform4 = translation_matrix(snowman1_x + 0.65, min_y-0.7_f64.tan()*0.75 + 0.107, snowman1_z - 1.95)
        * rotation_around_axis(Vec3::Y, -PI/2.0, Vec3::NULL)
        * shear_matrix_y(0.7, 0.0)
        * rotation_around_axis(Vec3::Y, PI/2.0, Vec3::NULL)
        * scale_matrix(0.75, 0.12, 0.75, Vec3::NULL);
    chair_bottom.apply_transform(&transform4);


    let td = 0.4;
    let td_angle = 30.0_f64.to_radians();
    let table_top = Cilinder::new(
        0.5, 0.1,
        Vec3::new(umbrella_pole_cb.x, 0.6, umbrella_pole_cb.z - 1.0),
        Vec3::Y,
        chair_material, true, true
    );

    let table_leg1 = Cilinder::new(
        0.075, 0.6,
        Vec3::new(umbrella_pole_cb.x, 0.0, umbrella_pole_cb.z - 1.0 + td),
        Vec3::Y,
        chair_material, true, true
    );

    let table_leg2 = Cilinder::new(
        0.075, 0.6,
        Vec3::new(umbrella_pole_cb.x + td*td_angle.cos(), 0.0, umbrella_pole_cb.z - 1.0 - td*td_angle.sin()),
        Vec3::Y,
        chair_material, true, true
    );

    let table_leg3 = Cilinder::new(
        0.075, 0.6,
        Vec3::new(umbrella_pole_cb.x - td*td_angle.cos(), 0.0, umbrella_pole_cb.z - 1.0 - td*td_angle.sin()),
        Vec3::Y,
        chair_material, true, true
    );

    let input = BufReader::new(File::open("objects/teapot400.obj").unwrap());
    let model: Obj = obj::load_obj(input).unwrap();
    let teapot_vertices: Vec<Vec3> = model.vertices
        .into_iter()
        .map(|vertex| Vec3::new(vertex.position[0] as f64, vertex.position[1] as f64, vertex.position[2] as f64))
        .collect();
    let teapot_triangles: Vec<[usize; 3]> = model.indices
        .chunks(3)
        .map(|face| [face[0] as usize, face[1] as usize, face[2] as usize])
        .collect();

    let mut teapot = Mesh::new(teapot_vertices, teapot_triangles, Material::WHITE);
    // let teapot_trans = rotation_around_axis(Vec3::Y, PI*0.5); // girar ao redor do eixo Y
    // teapot.apply_transform(&teapot_trans);
    teapot.scale(Vec3::all(0.1));
    teapot.translate(Vec3::new(umbrella_pole_cb.x + td/2.0, 0.7, umbrella_pole_cb.z - 1.0));

    let cup = Cilinder::new(
        0.065, 0.2,
        Vec3::new(umbrella_pole_cb.x - td/2.0, 0.7, umbrella_pole_cb.z - 1.0),
        Vec3::Y,
        Material::WHITE, true, false
    );

    let tea_material = Material::new(
        Vec3::new(0.8, 0.95, 0.7),
        Vec3::new(0.8, 0.95, 0.7),
        Vec3::new(0.8, 0.95, 0.7),
        40.0,
    );
    let liquid = Cilinder::new(
        0.064, 0.18,
        Vec3::new(umbrella_pole_cb.x - td/2.0, 0.7, umbrella_pole_cb.z - 1.0),
        Vec3::Y,
        tea_material, true, true
    );

    let sand_texture = Texture::new("textures/sand.png");
    let water_texture = Texture::new("textures/water.png");

    // snowman 2
    let snowman2_feet_center = Vec3::new(13.5, -0.10, 1.0);
    let snowman2_feet_radius = 0.65;
    
    let snowman2_torso_center = Vec3::new(13.5, -0.10 + snowman2_feet_radius, 0.9);
    let snowman2_torso_radius = 0.45;
    
    let hat2_material = Material::new(
        Vec3::new(0.82, 0.62, 0.25),
        Vec3::new(0.82, 0.62, 0.25),
        Vec3::all(0.5),
        3.0, 
    );
    let hat2_height = 0.3;
    let hat2_radius = 0.5;
    let mut hat2_direction = (snowman2_torso_center - snowman2_feet_center).normalized();
    let hat2_rotation_matrix = rotation_around_axis(Vec3::X, -PI/8.0, Vec3::NULL);
    hat2_direction.transform(&hat2_rotation_matrix);
    let hat2_center = snowman2_torso_center + hat2_direction * 0.35;
    
    let mut snowman2_nose_direction = hat2_direction;
    let snowman2_rotation_matrix = rotation_around_axis(Vec3::X, PI/2.0, Vec3::NULL);
    snowman2_nose_direction.transform(&snowman2_rotation_matrix);
    let snowman2_nose_cb = snowman2_torso_center + snowman2_nose_direction * snowman2_torso_radius;

    let snowman2_leye_center = snowman2_torso_center 
        + snowman2_nose_direction * snowman2_torso_radius
        - snowman1_eye_radius * hat2_direction / 2.0
        - Vec3::new(0.0, 0.0, snowman1_eye_radius/2.0)
        - Vec3::new(snowman1_eye_radius, 0.0, 0.0) * 3.0;

    let snowman2_reye_center = snowman2_torso_center 
        + snowman2_nose_direction * snowman2_torso_radius
        + snowman1_eye_radius * hat2_direction
        - Vec3::new(0.0, 0.0, snowman1_eye_radius/2.0)
        + Vec3::new(snowman1_eye_radius, 0.0, 0.0) * 3.0;

    // snowman 3
    let snowman3_feet_center = Vec3::new(13.5, -0.10, 7.0);
    let snowman3_feet_radius = 0.55;

    let snowman3_torso_center = Vec3::new(13.5, -0.10 + snowman3_feet_radius, 7.1);
    let snowman3_torso_radius = 0.45;
    
    let snowman3_head_center = Vec3::new(13.5, -0.10 + snowman3_feet_radius + snowman3_torso_radius, 7.0);
    let snowman3_head_radius = 0.30;

    let snowman3_up_direction = snowman3_head_center - snowman3_torso_center;
    let mut snowman3_nose_direction = snowman3_up_direction.normalized();
    let snowman3_rotation_matrix = rotation_around_axis(Vec3::X, -PI/2.0, Vec3::NULL);
    snowman3_nose_direction.transform(&snowman3_rotation_matrix);
    let snowman3_nose_cb = snowman3_head_center + snowman3_nose_direction * snowman3_head_radius;

    let snowman3_leye_center = snowman3_head_center 
        + snowman3_nose_direction * snowman3_head_radius
        + snowman1_eye_radius * hat2_direction
        + Vec3::new(0.0, 0.0, snowman1_eye_radius) * 1.5
        + Vec3::new(snowman1_eye_radius, 0.0, 0.0) * 3.0;

    let snowman3_reye_center = snowman3_head_center 
        + snowman3_nose_direction * snowman3_head_radius
        + snowman1_eye_radius * hat2_direction
        + Vec3::new(0.0, 0.0, snowman1_eye_radius) * 1.5
        - Vec3::new(snowman1_eye_radius, 0.0, 0.0) * 3.0;

    let ball_radius = 0.6; // Raio em metros
    let ball_center = snowman2_nose_cb + snowman2_nose_direction * 3.0; // Coords. centro da esfera (metros)
    let ball_material = Material::new(
        Vec3::new(0.9, 0.9, 0.9), // Ambient
        Vec3::new(0.9, 0.9, 0.9), // Diffuse
        Vec3::new(0.9, 0.9, 0.9), // Specular
        10.0, // coeficiente de "brilho" ou "polimento"
    );

    let ball_texture = Texture::new("textures/beach_ball.png");

    let lightpole_cilinder_1 = Cilinder::new(
        0.1, 6.5,
        Vec3::new(16.0, 0.0, 4.0),
        Vec3::Y,
        chair_material,
        true, true
    );

    let lightpole_cilinder_2 = Cilinder::new(
        0.1, 2.0,
        Vec3::new(16.0, 6.5, 4.0),
        -Vec3::X,
        chair_material,
        true, true
    );

    let lightpole_sphere = Sphere::new(
        Vec3::new(16.0, 6.5, 4.0), 0.1,
        chair_material, None
    );

    let lightpole_cone = Cone::new(
        0.3, 0.25,
        Vec3::new(14.0, 6.25, 4.0),
        Vec3::Y,
        umbrella_pole_material,
        false
    );
    
    // Criando os objetos e as luzes
    let shapes = vec![
        Plane::new( sand_pc, sand_normal, sand_material, Some(sand_texture), 4.0, 4.0 ), // chão
        Plane::new( water_pc, water_normal, water_material, Some(water_texture), 16.0, 16.0 ), // fundo

        lightpole_cilinder_1,
        lightpole_sphere,
        lightpole_cilinder_2,
        lightpole_cone,

        // bola de praia
        Sphere::new( ball_center, ball_radius, ball_material, Some(ball_texture) ),

        // snowman 1
        Sphere::new( snowman1_feet_center, snowman1_feet_radius, snowball_material, None ), // pé
        Sphere::new( snowman1_torso_center, snowman1_torso_radius, snowball_material, None ), // torso
        Sphere::new( snowman1_head_center, snowman1_head_radius, snowball_material, None ), // cabeça

        Sphere::new( leye_center, snowman1_eye_radius, eye_material, None ), // left eye
        Sphere::new( reye_center, snowman1_eye_radius, eye_material, None ), // right eye

        Cone::new( nose_radius, nose_height, nose_cb, nose_direction, nose_material, true ), // nose

        Cilinder::new( hat1_base_radius, hat1_base_height, hat1_base_cb, hat1_direction, hat1_material, true, true ), // hat base
        Cilinder::new( hat1_body_radius, hat1_body_height, hat1_body_cb, hat1_direction, hat1_material, true, true ), // hat body
        Cilinder::new( hat1_ribbon_radius, hat1_ribbon_height, hat1_ribbon_cb, hat1_direction, hat1_ribbon_material, true, true ), // hat ribbon

        // snowman 2
        Sphere::new( snowman2_feet_center, snowman2_feet_radius, snowball_material, None ),
        Sphere::new( snowman2_torso_center, snowman2_torso_radius, snowball_material, None ),

        Cone::new(hat2_radius, hat2_height, hat2_center, hat2_direction, hat2_material, false),
        Cone::new( nose_radius, nose_height, snowman2_nose_cb, snowman2_nose_direction, nose_material, true ),

        Sphere::new( snowman2_leye_center, snowman1_eye_radius, eye_material, None ), // left eye
        Sphere::new( snowman2_reye_center, snowman1_eye_radius, eye_material, None ), // right eye

        // snowman 3
        Sphere::new( snowman3_feet_center, snowman3_feet_radius, snowball_material, None ),
        Sphere::new( snowman3_torso_center, snowman3_torso_radius, snowball_material, None ),
        Sphere::new( snowman3_head_center, snowman3_head_radius, snowball_material, None ),
        Cone::new( nose_radius, nose_height, snowman3_nose_cb, snowman3_nose_direction, nose_material, true ),
        Sphere::new( snowman3_leye_center, snowman1_eye_radius, eye_material, None ), // left eye
        Sphere::new( snowman3_reye_center, snowman1_eye_radius, eye_material, None ), // right eye

        // umbrella
        Cilinder::new( umbrella_pole_radius, umbrella_pole_height, umbrella_pole_cb, umbrella_direction, umbrella_pole_material, true, true ),
        Cone::new( umbrella_top_radius, umbrella_top_height, umbrella_top_cb, umbrella_direction, umbrella_top_material, false ),

        // chair
        chair_middle.into_shape(),
        chair_top.into_shape(),
        chair_bottom.into_shape(),
        chair_leg_back.into_shape(),

        // table cup and teapot
        table_top,
        table_leg1,
        table_leg2,
        table_leg3,
        cup,
        liquid,
        teapot.into_shape(),

        // cilinder_x, cilinder_y, cilinder_z
    ];

    // Definindo as propriedades das luzes
    let light1_direction = Vec3::new(-1.0, -2.0, 0.25).normalized();
    let light1_color = Vec3::new(1.0, 1.0, 1.0);
    let light1_intensity = 0.55;

    let lights = vec![
        Light::directional(light1_direction, light1_color, light1_intensity),
    ];
    
    // let ambient_light = Vec3::all(0.2); // Luz ambiente
    let ambient_light = Vec3::all(0.4); // Luz ambiente
    let bg_color = Vec3::new(0.35,0.63,0.95); // cor do background

    let scene = Scene::new(shapes, lights, ambient_light, bg_color);

    let p0 = Vec3::new(2.3, 1.3, 3.9); // posição do observador
    let aspect_ratio: f64 = 16.0/9.0; // aspect ratio que eu quero
    let image_width: u32 = 960; // Resolução da imagem (número de colunas e linhas na grade)
    let image_height: u32 = ((image_width as f64)/aspect_ratio) as u32;
    let viewport_width: f64 = 16.0; // Tamanho da janela (em metros)
    let viewport_height: f64 = viewport_width/aspect_ratio;
    let focal_distance: f64 = 5.0; // distância da janela até o observador
    
    let mut camera: Camera = Camera::new(
        p0, // a posição do observador
        image_width, image_height, // número de colunas e linhas na grade (basicamente a resolução)
        viewport_width, viewport_height, // tamanho da janela (em metros)
        focal_distance, // distância da janela até o observador (em metros)
    );

    camera.rotate(Vec3::Y, PI + PI/3.0);

    (scene, camera, image_width, image_height)
}