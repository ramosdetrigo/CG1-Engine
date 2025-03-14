#![allow(clippy::new_ret_no_self)]
mod material;
pub use material::Material; 
mod shape;
pub use shape::Shape;
mod sphere;
pub use sphere::Sphere;
mod plane;
pub use plane::Plane;
mod cilinder;
pub use cilinder::Cilinder;
mod cone;
pub use cone::Cone;
mod mesh;
pub use mesh::Mesh;
mod texture;
pub use texture::Texture;