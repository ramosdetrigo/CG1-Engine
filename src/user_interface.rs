#![allow(static_mut_refs)]
use imgui::{TreeNodeFlags, Ui};
use std::f64::consts::PI;

use crate::engine::{Scene, Light};
use crate::engine::camera::Camera;
use crate::utils::transform::*;
use crate::utils::Matrix4;
use crate::utils::Vec3;
use crate::engine::shapes::{Cilinder, Cone, Mesh, Plane, Sphere};

static mut TRANSFORMATION_TYPE: i32 = 0;
static mut TRANSLATION: [f32; 3] = [0.0, 0.0, 0.0];
static mut SCALE: [f32; 3] = [0.0, 0.0, 0.0];
static mut ROTATION_AXIS: [f32; 3] = [0.0, 0.0, 0.0];
static mut ROTATION_ANGLE: f32 = 0.0;
static mut SH_X: [f32; 2] = [0.0, 0.0]; // yz zy
static mut SH_Y: [f32; 2] = [0.0, 0.0]; // xz zx
static mut SH_Z: [f32; 2] = [0.0, 0.0]; // xy yx
static mut SH_X_ANGLE: [f32; 2] = [0.0, 0.0]; // yz zy
static mut SH_Y_ANGLE: [f32; 2] = [0.0, 0.0]; // xz zx
static mut SH_Z_ANGLE: [f32; 2] = [0.0, 0.0]; // xy yx
static mut HOUSEHOLDER_P0: [f32; 3] = [0.0, 0.0, 0.0];
static mut HOUSEHOLDER_NORMAL: [f32; 3] = [0.0, 0.0, 0.0];
static mut TRANSFORM_MATRIX: Matrix4 = Matrix4::I;
static mut LOOK_AT: [f32; 3] = [0.0, 0.0, 0.0];
static mut UP: [f32; 3] = [0.0, 1.0, 0.0];


pub fn make_transformation_menu(ui: &Ui) {
    let mut transformation_type = unsafe { TRANSFORMATION_TYPE }; // 0: translation, 1: scale, 2: rotation, 3: shear 4: shear angle 5: reflection
    
    unsafe { ui.window("Transformation Menu")
        .collapsed(true, imgui::Condition::FirstUseEver)
        .size([300.0, 400.0], imgui::Condition::FirstUseEver)
        .position([400.0, 0.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.input_float3("\"look-at\" point", &mut LOOK_AT).build();
            ui.input_float3("\"look-at\" up vector", &mut UP).build();

            ui.separator();
            ui.text("Select a transformation type:");
            if ui.button("Translation") { transformation_type = 0; }
            if ui.button("Scale") { transformation_type = 1; }
            if ui.button("Rotation") { transformation_type = 2; }
            if ui.button("Shear") { transformation_type = 3; }
            if ui.button("Shear (angle)") { transformation_type = 4; }
            if ui.button("Reflection") { transformation_type = 5; }
            TRANSFORMATION_TYPE = transformation_type;

            match transformation_type {
                0 => {
                    ui.text("Translation:");
                    ui.input_float3("Translation", &mut TRANSLATION).build();
                },
                1 => {
                    ui.text("Scale:");
                    ui.input_float3("Scale", &mut SCALE).build();
                },
                2 => {
                    ui.text("Rotation:");
                    ui.input_float3("Axis", &mut ROTATION_AXIS).build();
                    ui.input_float("Angle", &mut ROTATION_ANGLE).build();
                },
                3 => {
                    ui.text("Shear:");
                    ui.input_float2("Shear YZ, ZY", &mut SH_X).build();
                    ui.input_float2("Shear XZ, ZX", &mut SH_Y).build();
                    ui.input_float2("Shear XY, YX", &mut SH_Z).build();
                },
                4 => {
                    ui.text("Shear (angle):");
                    ui.input_float2("Shear YZ, ZY", &mut SH_X_ANGLE).build();
                    ui.input_float2("Shear XZ, ZX", &mut SH_Y_ANGLE).build();
                    ui.input_float2("Shear XY, YX", &mut SH_Z_ANGLE).build();
                },
                5 => {
                    ui.text("Reflection: ");
                    ui.input_float3("Plane p0", &mut HOUSEHOLDER_P0).build();
                    ui.input_float3("Plane normal", &mut HOUSEHOLDER_NORMAL).build();
                }
                _ => {},
            }

            if ui.button("Update Transformation") {
                match transformation_type {
                    0 => TRANSFORM_MATRIX = translation_matrix(TRANSLATION[0] as f64, TRANSLATION[1] as f64, TRANSLATION[2] as f64),
                    1 => TRANSFORM_MATRIX = scale_matrix(SCALE[0] as f64, SCALE[1] as f64, SCALE[2] as f64),
                    2 => TRANSFORM_MATRIX = rotation_around_axis(Vec3::new(ROTATION_AXIS[0] as f64, ROTATION_AXIS[1] as f64, ROTATION_AXIS[2] as f64), ROTATION_ANGLE as f64),
                    3 => TRANSFORM_MATRIX = shear_matrix_x(SH_X[0] as f64, SH_X[1] as f64) * shear_matrix_y(SH_Y[0] as f64, SH_Y[1] as f64) * shear_matrix_z(SH_Z[0] as f64, SH_Z[1] as f64),
                    4 => TRANSFORM_MATRIX = shear_matrix_x_angle(SH_X_ANGLE[0] as f64, SH_X_ANGLE[1] as f64) * shear_matrix_y_angle(SH_Y_ANGLE[0] as f64, SH_Y_ANGLE[1] as f64) * shear_matrix_z_angle(SH_Z_ANGLE[0] as f64, SH_Z_ANGLE[1] as f64),
                    5 => TRANSFORM_MATRIX = householder_reflection(Vec3::new(HOUSEHOLDER_P0[0] as f64, HOUSEHOLDER_P0[1] as f64, HOUSEHOLDER_P0[2] as f64), Vec3::new(HOUSEHOLDER_NORMAL[0] as f64, HOUSEHOLDER_NORMAL[1] as f64, HOUSEHOLDER_NORMAL[2] as f64)),
                    _ => {}
                }
            }

            ui.text(format!("Transformation matrix:\n[{:.2} {:.2} {:.2} {:.2}]\n[{:.2} {:.2} {:.2} {:.2}]\n[{:.2} {:.2} {:.2} {:.2}]\n[{:.2} {:.2} {:.2} {:.2}]",
            TRANSFORM_MATRIX[0][0], TRANSFORM_MATRIX[0][1], TRANSFORM_MATRIX[0][2], TRANSFORM_MATRIX[0][3],
            TRANSFORM_MATRIX[1][0], TRANSFORM_MATRIX[1][1], TRANSFORM_MATRIX[1][2], TRANSFORM_MATRIX[1][3],
            TRANSFORM_MATRIX[2][0], TRANSFORM_MATRIX[2][1], TRANSFORM_MATRIX[2][2], TRANSFORM_MATRIX[2][3],
            TRANSFORM_MATRIX[3][0], TRANSFORM_MATRIX[3][1], TRANSFORM_MATRIX[3][2], TRANSFORM_MATRIX[3][3]));
        })};
}

fn mod_point(ui: &Ui, label: String, point: &mut Vec3, transform: bool) -> bool {
    let mut vec = [point.x as f32, point.y as f32, point.z as f32];
    if ui.input_float3(label, &mut vec).enter_returns_true(true).build() {
        point.x = vec[0] as f64; point.y = vec[1] as f64; point.z = vec[2] as f64;
        return true;
    }
    if transform {
        ui.same_line();
        unsafe { if ui.small_button("trans.") { point.transform(&TRANSFORM_MATRIX); } };
    }
    false
}

fn mod_dr(ui: &Ui, label: String, dr: &mut Vec3, p0: Option<Vec3>) -> bool {
    let mut changed = false;
    let mut vec = [dr.x as f32, dr.y as f32, dr.z as f32];
    if ui.input_float3(label, &mut vec).enter_returns_true(true).build() {
        dr.x = vec[0] as f64; dr.y = vec[1] as f64; dr.z = vec[2] as f64;
        changed = true;
    }

    unsafe { if let Some(p) = p0 {
        ui.same_line();
        if ui.small_button("look_at") {
            let look_at = Vec3::new(LOOK_AT[0] as f64, LOOK_AT[1] as f64, LOOK_AT[2] as f64);
            *dr = (look_at - p).normalized();
        }
    }};
    
    changed
}

fn mod_double(ui: &Ui, label: String, d: &mut f64) -> bool {
    let mut v = *d as f32;
    if ui.input_float(label, &mut v).enter_returns_true(true).build() {
        *d = v as f64;
        return true;
    }
    false
}

/// Retorna TRUE se deletou um objeto etc bla bla bla
fn mod_shape(ui: &Ui, scene: &mut Scene, index: usize, custom_label: Option<&str>) -> bool {
    let label = match custom_label {
        None => format!("{index}. "),
        Some(s) => s.to_string()
    };
    let shape = &mut scene.shapes[index];

    if let Some(sphere) = shape.as_any().downcast_mut::<Sphere>() {
        ui.text("Type: sphere");
        mod_point(ui, label.clone() + "center", &mut sphere.center, false);
        mod_double(ui, label.clone() + "radius", &mut sphere.radius);
        unsafe { if ui.small_button("transform" ) { shape.transform(&TRANSFORM_MATRIX); }; };
        if ui.button("delete") { scene.remove_shape(index); return true; }
    } else if let Some(plane) = shape.as_any().downcast_mut::<Plane>() {
        ui.text("Type: plane");
        mod_point(ui, label.clone() + "pc", &mut plane.pc, false);
        mod_point(ui, label.clone() + "normal", &mut plane.normal, false);
        unsafe { if ui.small_button("transform" ) { shape.transform(&TRANSFORM_MATRIX); }; };
        if ui.button("delete") { scene.remove_shape(index); return true; }
    } else if let Some(cilinder) = shape.as_any().downcast_mut::<Cilinder>() {
        ui.text("Type: cilinder");
        ui.text(format!(" - cb: {:.2?}\n - ct: {:.2?}\n - dc: {:.2?}\n - height: {:.2}", cilinder.cb, cilinder.ct, cilinder.dc, cilinder.h));
        mod_double(ui, label.clone() + "radius", &mut cilinder.r);
        unsafe { if ui.small_button("transform" ) { shape.transform(&TRANSFORM_MATRIX); }; };
        if ui.button("delete") { scene.remove_shape(index); return true; }
    } else if let Some(cone) = shape.as_any().downcast_mut::<Cone>() {
        ui.text("Type: cone");
        ui.text(format!(" - cb: {:.2?}\n - v: {:.2?}\n - dc: {:.2?}\n - height: {:.2}", cone.cb, cone.v, cone.dc, cone.h));
        mod_double(ui, label.clone() + "radius", &mut cone.r);
        unsafe { if ui.small_button("transform" ) { shape.transform(&TRANSFORM_MATRIX); }; };
        if ui.button("delete") { scene.remove_shape(index); return true; }
    } else if let Some(mesh) = shape.as_any().downcast_mut::<Mesh>() {
        ui.text("Type: mesh");
        ui.text(format!(" - centroid: {:.2?}", mesh.centroid));
        unsafe { if ui.small_button("transform" ) { shape.transform(&TRANSFORM_MATRIX); }; };
        if ui.button("delete") { scene.remove_shape(index); return true; }
    }
    false
}

pub fn make_ui(ui: &mut Ui, scene: &mut Scene, camera: &mut Camera, selected_shape: &mut Option<usize>) {
    ui.window("User interface")
    .collapsed(true, imgui::Condition::FirstUseEver)
    .size([400.0, 540.0], imgui::Condition::FirstUseEver)
    .position([0.0, 0.0], imgui::Condition::FirstUseEver)
    .build(|| {
        if ui.button("make day") { make_day(scene); }
        if ui.button("make night") { make_night(scene); }
        
        // Menu de camera
        if ui.collapsing_header("Camera", TreeNodeFlags::empty()) {
            let mut cpos = camera.pos;
            if mod_point(ui, "Camera position".to_string(), &mut cpos, false) { camera.set_position(cpos); }
            
            let mut whf = [camera.viewport.width as f32, camera.viewport.height as f32, camera.focal_distance as f32];
            if ui.input_float3("Width, Height, Focal distance", &mut whf).enter_returns_true(true).build() {
                camera.set_viewport_size(whf[0] as f64, whf[1] as f64);
                camera.set_focal_distance(whf[2] as f64);
            }

            unsafe { if ui.small_button("look_at") {
                let look_at = Vec3::new(LOOK_AT[0] as f64, LOOK_AT[1] as f64, LOOK_AT[2] as f64);
                let up = Vec3::new(UP[0] as f64, UP[1] as f64, UP[2] as f64);
                camera.look_at(look_at, up);
            }};

            ui.text(format!("Camera coord system:\nX: {:.2?}\nY: {:.2?}\nZ: {:.2?}", camera.coord_system[0], camera.coord_system[1], camera.coord_system[2]));
            ui.separator();
            
            match selected_shape {
                None => {ui.text("No shape selected.")}
                Some(shape_index) => {
                    ui.text(format!("Selected shape: {shape_index}"));
                    if mod_shape(ui, scene, *shape_index, Some("Sel. ")) { *selected_shape = None; }
                }
            }
        }
        
        // Menu de luzes
        let mut counter = 1;
        if ui.collapsing_header("Luzes", TreeNodeFlags::empty()) {
            let mut delete: Option<usize> = None;
            for light in &mut scene.lights {
                match light {
                    Light::Point { pos, intensity } => {
                        let name = format!("{counter}. LUZ PONTUAL");
                        ui.text(format!("{name}"));
                        mod_point(ui, format!("{counter}. Posição"), pos, true);
                        mod_point(ui, format!("{counter}. Intensidade"), intensity, false);
                    }
                    Light::Spotlight { pos, dr, angle, intensity } => {
                        let name = format!("{counter}. LUZ SPOT");
                        ui.text(format!("{name}"));
                        mod_point(ui, format!("{counter}. Posição"), pos, true);
                        mod_dr(ui, format!("{counter}. Direção"), dr, Some(*pos));
                        mod_double(ui, format!("{counter}. Ângulo"), angle);
                        mod_point(ui, format!("{counter}. Intensidade"), intensity, false);
                    }
                    Light::Directional { dr, intensity } => {
                        let name = format!("{counter}. LUZ DIRECIONAL");
                        ui.text(format!("{name}"));
                        mod_dr(ui, format!("{counter}. Direção"), dr, None);
                        mod_point(ui, format!("{counter}. Intensidade"), intensity, false);
                    }
                }
                if ui.button("delete") { delete = Some(counter-1); }
                counter += 1;
            }
            if let Some(n) = delete { scene.remove_light(n); }
        }
    });

    make_transformation_menu(ui);
}

fn make_day(scene: &mut Scene) {
    let light1_direction = Vec3::new(-1.0, -2.0, 0.25).normalized();
    let light1_color = Vec3::new(1.0, 1.0, 1.0);
    let light1_intensity = 0.55;

    let bg_color = (Vec3::new(0.35,0.63,0.95) * 255.0).clamp(0.0, 255.0); // cor do background
    let ambient_light = Vec3::all(0.4); // Luz ambiente
    let lights = vec![
        Light::directional(light1_direction, light1_color, light1_intensity),
    ];

    scene.bg_color = bg_color;
    scene.ambient_light = ambient_light;
    scene.lights = lights;
}

fn make_night(scene: &mut Scene) {
    let snowman1_x = 3.0;
    let snowman1_z = 7.0;
    let umbrella_top_height = 0.5;
    let umbrella_direction = Vec3::new(0.2, 1.0, 0.0);
    let umbrella_pole_height = 3.1;
    let umbrella_pole_cb = Vec3::new(snowman1_x + 2.0, -0.20, snowman1_z);
    let umbrella_top_cb = umbrella_pole_cb + umbrella_direction*(umbrella_pole_height-umbrella_top_height-0.037);

    let light2_color = Vec3::new(1.0, 0.8, 0.3);
    let light2_intensity = 0.65;
    let light3_position = umbrella_top_cb + (umbrella_top_height * Vec3::Y * 0.9);
    let ambient_light = Vec3::all(0.1); // Luz ambiente
    let bg_color = Vec3::NULL;
    
    let lights = vec![
        Light::spotlight(
            Vec3::new(14.0, 6.25, 4.0),
            -Vec3::Y, PI/4.0,
            light2_color,
            light2_intensity
        ),
        Light::point( light3_position, light2_color, 0.3 ),
    ];

    scene.bg_color = bg_color;
    scene.ambient_light = ambient_light;
    scene.lights = lights;
}