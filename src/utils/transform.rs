#![allow(unused_imports)]
use super::{matrix4, Matrix3, Matrix4, Vec3, Vec4};

// pub trait Transformable {
//     fn apply_transform(&mut self, transform: Matrix4);
//     fn translate(&mut self, translation_vector: Vec3);
//     fn scale();
//     fn rotate(&mut self, rotation_vector: Vec3);
// }

pub fn translation_matrix(tx: f64, ty: f64, tz: f64) -> Matrix4 {
    Matrix4::new([
        [1.0, 0.0, 0.0, tx],
        [0.0, 1.0, 0.0, ty],
        [0.0, 0.0, 1.0, tz],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scale_matrix(sx: f64, sy: f64, sz: f64) -> Matrix4 {
    Matrix4::new([
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, sz, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_x(sh_yz: f64, sh_zy: f64) -> Matrix4 {
    Matrix4::new([
        [1.0, sh_yz, sh_zy, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_y(sh_xz: f64, sh_zx: f64) -> Matrix4 {
    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [sh_xz, 1.0, sh_zx, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_z(sh_xy: f64, sh_yx: f64) -> Matrix4 {
    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sh_xy, sh_yx, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_x_angle(angle: f64) -> Matrix4 {
    let sh_yz = angle.tan(); // Shear factor based on the angle for y-axis

    Matrix4::new([
        [1.0, sh_yz, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_y_angle(angle: f64) -> Matrix4 {
    let sh_xz = angle.tan(); // Shear factor based on the angle for x-axis

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [sh_xz, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_z_angle(angle: f64) -> Matrix4 {
    let sh_xy = angle.tan(); // Shear factor based on the angle for x-axis

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sh_xy, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}
