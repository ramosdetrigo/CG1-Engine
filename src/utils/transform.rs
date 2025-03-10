use super::{Matrix4, Vec3};

pub fn translation_matrix(tx: f64, ty: f64, tz: f64) -> Matrix4 {
    Matrix4::new([
        [1.0, 0.0, 0.0, tx],
        [0.0, 1.0, 0.0, ty],
        [0.0, 0.0, 1.0, tz],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scale_matrix(sx: f64, sy: f64, sz: f64, pc: Vec3) -> Matrix4 {
    let scale_matrix = Matrix4::new([
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, sz, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    translation_matrix(pc.x, pc.y, pc.z) * scale_matrix * translation_matrix(-pc.x, -pc.y, -pc.z)
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

pub fn shear_matrix_x_angle(angle_yz: f64, angle_zy: f64) -> Matrix4 {
    let sh_yz = angle_yz.tan(); // Shear factor based on the angle for x-y plane
    let sh_zy = angle_zy.tan(); // Shear factor based on the angle for x-y plane

    Matrix4::new([
        [1.0, sh_yz, sh_zy, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_y_angle(angle_xz: f64, angle_zy: f64) -> Matrix4 {
    let sh_xz = angle_xz.tan(); // Shear factor based on the angle for y-x plane
    let sh_zy = angle_zy.tan(); // Shear factor based on the angle for y-z plane

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [sh_xz, 1.0, sh_zy, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_matrix_z_angle(angle_xy: f64, angle_yx: f64) -> Matrix4 {
    let sh_xy = angle_xy.tan(); // Shear factor based on the angle for z-x plane
    let sh_yx = angle_yx.tan(); // Shear factor based on the angle for z-y plane

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sh_xy, sh_yx, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_around_axis(axis: Vec3, angle: f64, pc: Vec3) -> Matrix4 {
    let axis = axis.normalized();
    let (x, y, z) = (axis.x, axis.y, axis.z);
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    let one_minus_cos = 1.0 - cos_theta;

    let rotation_matrix = Matrix4::new([
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
    ]);

    translation_matrix(pc.x, pc.y, pc.z) * rotation_matrix * translation_matrix(-pc.x, -pc.y, -pc.z)
}

pub fn householder_reflection(pc: Vec3, normal: Vec3) -> Matrix4 {
    // Normalize the normal vector
    let norm = normal.length();
    let u = normal / norm;

    // Calculate the Householder matrix
    let uu_t = [
        [u.x * u.x, u.x * u.y, u.x * u.z, 0.0],
        [u.y * u.x, u.y * u.y, u.y * u.z, 0.0],
        [u.z * u.x, u.z * u.y, u.z * u.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    // Create the Householder matrix
    let mut householder_matrix = Matrix4::I;

    for i in 0..3 {
        for j in 0..3 {
            householder_matrix[i][j] -= 2.0 * uu_t[i][j];
        }
    }

    translation_matrix(pc.x, pc.y, pc.z) * householder_matrix * translation_matrix(-pc.x, -pc.y, -pc.z)
}

