use crate::utils::Vec3;

#[derive(Clone, PartialEq)]
/// Raio de origem `origin` e direção `dr` (Função `R(t) = p0 + t*dr`) 
pub struct Ray {
    pub origin: Vec3, // Origem do raio
    pub dr: Vec3, // Direção do raio
}

impl Ray {
    #[inline]
    #[must_use]
    /// Cria um novo raio de origem `origin` e direção `dr` \
    /// (se `dr` for normalizado, `t` é a distância entre a origem e `R(t)`)
    pub fn new(origin: Vec3, dr: Vec3) -> Ray {
        Ray { origin, dr }
    }
    
    /// função `R(t) = p0 + t*dr`. \
    /// retorna o ponto `P` em `R(t)` (se `dr` for normalizado, `t` é a distância entre a origem e `R(t)`)
    #[inline]
    #[must_use]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t*self.dr
    }
}   