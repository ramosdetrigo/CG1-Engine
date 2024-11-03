use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Material de um objeto. \
/// `k_amb`: Coeficiente de reflexão de luz ambiente \
/// `k_dif`: Coeficiente de reflexão difusa \
/// `k_esp`: Coeficiente de reflexão especular \
/// `e`: Coeficiente de "brilho" ou "polimento"
pub struct Material {
    pub k_amb: Vec3,
    pub k_dif: Vec3,
    pub k_esp: Vec3,
    pub e: f32,
}

impl Material {
    #[inline]
    #[must_use]
    /// Cria um novo material de um objeto. \
    /// `k_amb`: Coeficiente de reflexão de luz ambiente \
    /// `k_dif`: Coeficiente de reflexão difusa \
    /// `k_esp`: Coeficiente de reflexão especular \
    /// `e`: Coeficiente de "brilho" ou "polimento"
    pub fn new(k_amb: Vec3, k_dif: Vec3, k_esp: Vec3, e: f32) -> Self {
        Self{ k_amb, k_dif, k_esp, e }
    }
}