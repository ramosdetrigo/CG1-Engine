use crate::engine::{Camera, Scene, Light};
use crate::utils::Vec3;
use crate::engine::shapes::{Material, Sphere, Plane, Cilinder, Cone};

pub fn beach() -> (Scene, Camera, u32, u32) {
    let p0 = Vec3::new(0.0, 0.5, 0.0); // posição do observador
    
    let aspect_ratio: f32 = 16.0/9.0; // aspect ratio que eu quero

    let image_width: u32 = 960; // Resolução da imagem (número de colunas e linhas na grade)
    let image_height: u32 = ((image_width as f32)/aspect_ratio) as u32;
    
    let viewport_width: f32 = 0.032; // Tamanho da janela (em metros)
    let viewport_height: f32 = viewport_width/aspect_ratio;
    let viewport_distance: f32 = 0.01; // distância da janela até o observador
    
    let bg_color = Vec3::new(0.35,0.63,0.95); // cor do background

    let plane1_pc = Vec3::new(-1.0, -0.5, -2.5); // Ponto conhecido do plano
    let plane1_normal = Vec3::new(0.0, 1.0 ,-0.01); // Normal do plano
    let plane1_material = Material::new(
        Vec3::new(0.92, 0.78, 0.65),
        Vec3::new(0.92, 0.78, 0.65),
        Vec3::all(0.5),
        3.0, 
    );

    let plane2_pc = Vec3::new(0.0, -0.51, -3.5); // Ponto conhecido do plano
    let plane2_normal = Vec3::new(0.0, 1.0 ,0.0); // Normal do plano
    let plane2_material = Material::new(
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

    let snowball1_radius = 0.4;
    let snowball1_center = Vec3::new(-1.0, -0.5+snowball1_radius/1.5, -2.5);
    
    let snowball2_radius = 0.325;
    let snowball2_center = Vec3::new(-1.0, snowball1_center.y+snowball1_radius+snowball2_radius/2.0, -2.5);

    let snowball3_radius = 0.25;
    let snowball3_center = Vec3::new(-1.0, snowball2_center.y+snowball2_radius+snowball3_radius/2.0, -2.5);

    let hat_material = Material::new(
        Vec3::all(0.1),
        Vec3::all(0.1),
        Vec3::all(0.8),
        15.0,
    );
    let hat_direction = Vec3::new(0.0, 1.0, 0.0);

    let hat_base_radius = 0.3;
    let hat_base_height = 0.025;
    let hat_base_cb = Vec3::new(-1.0, snowball3_center.y+snowball3_radius-hat_base_height*2.0, -2.5);

    let hat_body_radius = 0.225;
    let hat_body_height = 0.4;
    let hat_body_cb = Vec3::new(-1.0, hat_base_cb.y+hat_base_height, -2.5);

    let hat_ribbon_material = Material::new(
        Vec3::new(0.6,0.05, 0.05),
        Vec3::new(0.6,0.05, 0.05),
        Vec3::new(0.6,0.05, 0.05),
        hat_material.e,
    );
    let hat_ribbon_radius = hat_body_radius + 0.001;
    let hat_ribbon_height = 0.05;
    let hat_ribbon_cb = Vec3::new(-1.0, hat_body_cb.y+0.05, -2.5);

    let nose_material = Material::new(
        Vec3::new(0.95,0.5, 0.15),
        Vec3::new(0.95,0.5, 0.15),
        Vec3::new(0.95,0.5, 0.15),
        3.0
    );
    let nose_radius = 0.05;
    let nose_height = 0.1;
    let nose_direction = Vec3::new(0.0, 0.0, 1.0);
    let nose_cb = Vec3::new(-1.0, snowball3_center.y, -2.5 + snowball3_radius);

    let eye_material = Material::new(
        Vec3::all(0.1),
        Vec3::all(0.1),
        Vec3::all(0.3),
        3.0,
    );
    let leye_radius = 0.05;
    let leye_center = Vec3::new(-1.0-nose_radius*2.0, nose_cb.y+leye_radius, nose_cb.z-leye_radius);
    
    let reye_radius = leye_radius;
    let reye_center = Vec3::new(-1.0+nose_radius*2.0, nose_cb.y+reye_radius, nose_cb.z-reye_radius);

    let umbrella_direction = Vec3::new(0.2, 1.0, 0.0);
    let umbrella_top_height = 0.1;

    let umbrella_pole_material = Material::new(
        Vec3::all(0.5),
        Vec3::all(0.5),
        Vec3::all(0.2),
        20.0
    );
    let umbrella_pole_radius = 0.05;
    let umbrella_pole_height = 2.1+umbrella_top_height;
    let umbrella_pole_cb = Vec3::new(0.25, -0.55, -2.5);

    let umbrella_top_material = Material::new(
        Vec3::new(0.9, 0.3, 0.3),
        Vec3::new(0.9, 0.3, 0.3),
        Vec3::new(0.9, 0.3, 0.3),
        3.0
    );
    let umbrella_top_radius = 1.0;
    let umbrella_top_cb = umbrella_pole_cb + umbrella_direction*(umbrella_pole_height-umbrella_top_height-0.1);


    // Definindo as propriedades das luzes
    let light1_pos = Vec3::new(0.0, 2.0, 1.0);
    let light1_color = Vec3::new(1.0, 1.0, 1.0);
    let light1_intensity = 1.0;

    
    // Criando os objetos e as luzes
    let shapes = vec![
        Plane::new( plane1_pc, plane1_normal, plane1_material ), // chão
        Plane::new( plane2_pc, plane2_normal, plane2_material ), // fundo
        
        Sphere::new( snowball1_center, snowball1_radius, snowball_material ), // ball1
        Sphere::new( snowball2_center, snowball2_radius, snowball_material ), // ball2
        Sphere::new( snowball3_center, snowball3_radius, snowball_material ), // ball3

        Cilinder::new( hat_base_radius, hat_base_height, hat_base_cb, hat_direction, hat_material, true, true ), // hat base
        Cilinder::new( hat_body_radius, hat_body_height, hat_body_cb, hat_direction, hat_material, true, true ), // hat body
        Cilinder::new( hat_ribbon_radius, hat_ribbon_height, hat_ribbon_cb, hat_direction, hat_ribbon_material, true, true ), // hat ribbon

        Cone::new( nose_radius, nose_height, nose_cb, nose_direction, nose_material, true ), // nose
        
        Sphere::new( leye_center, leye_radius, eye_material ), // left eye
        Sphere::new( reye_center, reye_radius, eye_material ), // right eye

        Cilinder::new( umbrella_pole_radius, umbrella_pole_height, umbrella_pole_cb, umbrella_direction, umbrella_pole_material, true, true ),
        Cone::new( umbrella_top_radius, umbrella_top_height, umbrella_top_cb, umbrella_direction, umbrella_top_material, false ),
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