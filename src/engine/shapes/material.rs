// TODO: k_amb k_dif k_esp are vectors, not floats.
// TODO: are they also the color? maybe having a color as an abstraction could be good but eh. idk.
// TODO: they being the color could also explain the specular reflection being white etc.
use crate::utils::vec::Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct Material {
    pub k_amb: Vec3,
    pub k_dif: Vec3,
    pub k_esp: Vec3,
    pub e: f32,
}

impl Material {
    pub fn new(k_amb: Vec3, k_dif: Vec3, k_esp: Vec3, e: f32) -> Self {
        Self{ k_amb, k_dif, k_esp, e }
    }
}