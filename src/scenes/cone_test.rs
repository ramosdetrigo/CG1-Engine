use std::f64::consts::PI;
use crate::engine::{Scene, Light};
use crate::engine::camera::Camera;
use crate::utils::Vec3;
use crate::engine::shapes::{Material, Sphere, Plane, Cilinder, Cone};

pub fn cone_test() -> (Scene, Camera, u32, u32) {
    // Definindo as propriedades de cada objeto
    let sphere1_radius = 0.65; // Raio em metros
    let sphere1_center = Vec3::new(-0.0, 1.41, -2.75); // Coords. centro da esfera (metros)
    let sphere1_material = Material::new(
        Vec3::new(0.7, 0.2, 0.2), // Ambient
        Vec3::new(0.7, 0.2, 0.2), // Diffuse
        Vec3::new(0.7, 0.2, 0.2), // Specular
        10.0, // coeficiente de "brilho" ou "polimento"
    );
    
    let plane1_pc = Vec3::new(0.0, -0.5, 0.0); // Ponto conhecido do plano
    let plane1_normal = Vec3::new(0.0, 1.0 ,0.0); // Normal do plano
    let plane1_material = Material::new(
        Vec3::all(0.4),
        Vec3::all(0.4),
        Vec3::all(0.0),
        3.0, 
    );

    let plane2_pc = Vec3::new(0.0, 0.0, -3.5); // Ponto conhecido do plano
    let plane2_normal = Vec3::new(0.0, 0.0 ,1.0); // Normal do plano
    let plane2_material = Material::new(
        Vec3::new(0.4, 0.4, 0.7),
        Vec3::new(0.4, 0.4, 0.7),
        Vec3::all(0.0),
        3.0, 
    );

    let cilinder1_cb = Vec3::new(1.85, -0.5, -3.0);
    let cilinder1_dc = Vec3::new(0.0, 1.0, 0.0);
    let cilinder1_r = 0.5;
    let cilinder1_h = 1.4;
    let cilinder1_material = Material::new(
        Vec3::new(0.2, 0.2, 0.9), // Ambient
        Vec3::new(0.2, 0.2, 0.9), // Diffuse
        Vec3::new(0.2, 0.2, 0.9), // Specular
        150.0, // coeficiente de "brilho" ou "polimento"
    );

    let cilinder2_cb = Vec3::new(-1.85, -0.5, -3.0);
    let cilinder2_dc = Vec3::new(0.0, 1.0, 0.0);
    let cilinder2_r = 0.5;
    let cilinder2_h = 1.4;
    let cilinder2_material = Material::new(
        Vec3::new(0.2, 0.2, 0.9), // Ambient
        Vec3::new(0.2, 0.2, 0.9), // Diffuse
        Vec3::new(0.2, 0.2, 0.9), // Specular
        150.0, // coeficiente de "brilho" ou "polimento"
    );

    let cone1_cb = Vec3::new(0.0, -0.5, -2.75);
    let cone1_dc = Vec3::new(0.0, 1.0, 0.0);
    let cone1_r = 0.65;
    let cone1_h = 1.26;
    let cone1_material = Material::new(
        Vec3::new(0.2, 0.9, 0.2), // Ambient
        Vec3::new(0.2, 0.9, 0.2), // Diffuse
        Vec3::new(1.0, 1.0, 1.0), // Specular
        500.0, // coeficiente de "brilho" ou "polimento"
    );

    
    
    // Definindo as propriedades das luzes
    let light1_pos = Vec3::new(0.0, 0.8, 0.0);
    let light1_color = Vec3::new(1.0, 1.0, 1.0);
    let light1_intensity = 1.0;
    
    // Criando os objetos e as luzes
    let shapes = vec![
        Plane::new( plane1_pc, plane1_normal, plane1_material ),
        Plane::new( plane2_pc, plane2_normal, plane2_material ),
        Cilinder::new( cilinder2_r, cilinder2_h, cilinder2_cb, cilinder2_dc, cilinder2_material, true, true),
        Sphere::new( sphere1_center, sphere1_radius, sphere1_material, None ),
        Cone::new( cone1_r, cone1_h, cone1_cb, cone1_dc, cone1_material, true),
        Cilinder::new( cilinder1_r, cilinder1_h, cilinder1_cb, cilinder1_dc, cilinder1_material, true, true),
    ];

    let lights = vec![
        Light::point( light1_pos, light1_color, light1_intensity ),
        // Light::spotlight(light1_pos, -Vec3::Y, PI/4.0, light1_color, light1_intensity),
        // Light::directional(-Vec3::Y, light1_color, light1_intensity),
    ];

    let ambient_light = Vec3::new(0.3, 0.3, 0.3); // Luz ambiente
    let scene = Scene::new(shapes, lights, ambient_light);

    let p0 = Vec3::new(0.0, 0.0, 0.0); // posição do observador
    let aspect_ratio: f64 = 16.0/9.0; // aspect ratio que eu quero
    let image_width: u32 = 960; // Resolução da imagem (número de colunas e linhas na grade)
    let image_height: u32 = ((image_width as f64)/aspect_ratio) as u32;
    let viewport_distance: f64 = 0.01; // distância da janela até o observador
    let bg_color = Vec3::new(0.0,0.0,0.0); // cor do background
    
    #[allow(unused_mut)]
    let mut camera: Camera = Camera::new(
        p0, // a posição do observador
        image_width, image_height, // número de colunas e linhas na grade (basicamente a resolução)
        1.6, 0.9, // tamanho da janela (em metros)
        viewport_distance, // distância da janela até o observador (em metros)
        bg_color, // cor do background
    );

    // camera.set_projection(crate::engine::camera::Projection::Ortographic);

    (scene, camera, image_width, image_height)
}