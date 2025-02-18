use super::{Matrix4, Vec3};

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
    let sh_yz = angle.tan(); // Shear factor based on the angle for x-axis

    Matrix4::new([
        [1.0, sh_yz, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_y_angle(angle: f64) -> Matrix4 {
    let sh_xz = angle.tan(); // Shear factor based on the angle for y-axis

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [sh_xz, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_z_angle(angle: f64) -> Matrix4 {
    let sh_xy = angle.tan(); // Shear factor based on the angle for z-axis

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sh_xy, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_around_axis(axis: Vec3, angle: f64) -> Matrix4 {
    let axis = axis.normalize();
    let (x, y, z) = (axis.x, axis.y, axis.z);
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    let one_minus_cos = 1.0 - cos_theta;

    Matrix4::new([
        [
            cos_theta + x * x * one_minus_cos,
            x * y * one_minus_cos - z * sin_theta,
            x * z * one_minus_cos + y * sin_theta,
            0.0,
        ],
        [
            y * x * one_minus_cos + z * sin_theta,
            cos_theta + y * y * one_minus_cos,
            y * z * one_minus_cos - x * sin_theta,
            0.0,
        ],
        [
            z * x * one_minus_cos - y * sin_theta,
            z * y * one_minus_cos + x * sin_theta,
            cos_theta + z * z * one_minus_cos,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ])
}