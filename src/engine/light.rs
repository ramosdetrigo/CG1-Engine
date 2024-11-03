use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Luz de posição `pos`, cor `color`, e intensidade `intensity`. \
/// (durante a renderização, a cor é o mesmo que `color * intensity`)
pub struct Light {
    pub pos: Vec3, // Posição da luz no cenário
    pub color: Vec3, // Cor da luz
    pub intensity: f32 // Intensidade da luz
}

impl Light {
    #[inline]
    #[must_use]
    /// Cria uma luz com posição `pos`, cor `color`, e intensidade `intensity`.
    pub fn new(pos: Vec3, color: Vec3, intensity: f32) -> Light {
        Light { pos, color, intensity }
    }
}