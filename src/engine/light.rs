use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
pub struct Light {
    pub pos: Vec3,
    pub color: Vec3,
    pub intensity: f32
}

impl Light {
    #[inline]
    #[must_use]
    pub fn new(pos: Vec3, color: Vec3, intensity: f32) -> Light {
        Light { pos, color, intensity }
    }
}