#![allow(unused_imports)]
use super::{matrix4, Matrix3, Matrix4, Vec3, Vec4};

pub fn translate_matrix(tx: f64, ty: f64, tz: f64) -> Matrix4 {
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

// QUESTÃO 1.1
// MATRIZ M1: translada pra origem de acordo com hx hy hz
// [1 0 0 -Hx]
// [0 1 0 -Hy]
// [0 0 1 -Hz]
// [0 0 0 1]

// MATRIZ M2: escala de acordo com sx sy sz
// [Sx 0 0 0]
// [0 Sy 0 0]
// [0 0 Sz 0]
// [0 0 0 1]

// MATRIZ M3: translada pra origem de acordo com hx hy hz
// [1 0 0 Hx]
// [0 1 0 Hy]
// [0 0 1 Hz]
// [0 0 0 1]


// QUESTÃO 1.2
// 1. ESCALA
// [4, 0, 0, 0],
// [0, 1, 0, 0],
// [0, 0, 0.2, 0],
// [0, 0, 0, 1],

// 2. CISALHAMENTO NO EIXO Y
// [1, 0, 0, 0],
// [3/4, 1, 0, 0], --> aproximadamente tan(37º)
// [0, 0, 1, 0],
// [0, 0, 0, 1],

// 3. TRANSLAÇÃO SEGUNDO O A = 4, 5, 3
// [1, 0, 0, 4],
// [0, 1, 0, 5],
// [0, 0, 1, 3],
// [0, 0, 0, 1],


// pub fn shear_x(sh_yz: f64, sh_zy: f64) -> Matrix4 {
//     Matrix4::new([
//         [1.0, sh_yz, sh_zy, 0.0],
//         [0.0, 1.0, 0.0, 0.0],
//         [0.0, 0.0, 1.0, 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ])
// }

// pub fn shear_y(sh_xz: f64, sh_zx: f64) -> Matrix4 {
//     Matrix4::new([
//         [1.0, 0.0, 0.0, 0.0],
//         [sh_xz, 1.0, sh_zx, 0.0],
//         [0.0, 0.0, 1.0, 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ])
// }

// pub fn shear_z(sh_xy: f64, sh_yx: f64) -> Matrix4 {
//     Matrix4::new([
//         [1.0, 0.0, 0.0, 0.0],
//         [0.0, 1.0, 0.0, 0.0],
//         [sh_xy, sh_yx, 1.0, 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ])
// }

pub fn shear_x(angle: f64) -> Matrix4 {
    let sh_yz = angle.tan(); // Shear factor based on the angle for y-axis

    Matrix4::new([
        [1.0, sh_yz, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_y(angle: f64) -> Matrix4 {
    let sh_xz = angle.tan(); // Shear factor based on the angle for x-axis

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [sh_xz, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shear_z(angle: f64) -> Matrix4 {
    let sh_xy = angle.tan(); // Shear factor based on the angle for x-axis

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sh_xy, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}
