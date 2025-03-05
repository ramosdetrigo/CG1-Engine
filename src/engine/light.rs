#![allow(dead_code)]
use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Luz de posição `pos`, cor `color`, e intensidade `intensity`. \
/// (durante a renderização, a cor é o mesmo que `color * intensity`)
pub enum Light {
    Point { pos: Vec3, intensity: Vec3 }, // Posição da luz no cenário
    Spotlight { pos: Vec3, dr: Vec3, angle: f64, intensity: Vec3 },
    Directional { dr: Vec3, intensity: Vec3 },
}

impl Light {
    #[inline]
    #[must_use]
    /// Cria uma point light com posição `pos`, e intensidade `intensity * color`.
    pub fn point(pos: Vec3, color: Vec3, intensity: f64) -> Light {
        Light::Point { pos, intensity: color*intensity }
    }

    #[inline]
    #[must_use]
    /// Cria uma spotlight com posição `pos`, direção `dr`, ângulo `angle`, e intensidade `intensity * color`.
    pub fn spotlight(pos: Vec3, dr: Vec3, angle: f64, color: Vec3, intensity: f64) -> Light {
        Light::Spotlight { pos, dr:-dr, angle, intensity: color*intensity }
    }

    #[inline]
    #[must_use]
    /// Cria uma luz com posição `pos`, cor `color`, e intensidade `intensity`.
    pub fn directional(dr: Vec3, color: Vec3, intensity: f64) -> Light {
        Light::Directional { dr:-dr, intensity: color*intensity }
    }
}