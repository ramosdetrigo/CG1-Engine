#![allow(dead_code)]
use crate::utils::Vec3;

#[derive(Clone, Copy, PartialEq)]
/// Material de um objeto. \
/// `k_amb`: Coeficiente de reflexão de luz ambiente \
/// `k_dif`: Coeficiente de reflexão difusa \
/// `k_esp`: Coeficiente de reflexão especular \
/// `e`: Coeficiente de "brilho" ou "polimento"
pub struct Material {
    pub k_amb: Vec3,
    pub k_dif: Vec3,
    pub k_esp: Vec3,
    pub e: f64,
}

impl Material {
    pub const WHITE: Material = Material {
        k_amb: Vec3::all(0.8),
        k_dif: Vec3::all(0.8),
        k_esp: Vec3::all(0.8),
        e: 10.0,
    };

    #[inline]
    #[must_use]
    /// Cria um novo material de um objeto. \
    /// `k_amb`: Coeficiente de reflexão de luz ambiente \
    /// `k_dif`: Coeficiente de reflexão difusa \
    /// `k_esp`: Coeficiente de reflexão especular \
    /// `e`: Coeficiente de "brilho" ou "polimento"
    pub fn new(k_amb: Vec3, k_dif: Vec3, k_esp: Vec3, e: f64) -> Self {
        Self{ k_amb, k_dif, k_esp, e }
    }
}