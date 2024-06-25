mod dielectric;
mod lambertian;
mod metal;

use std::sync::Arc;

use crate::{color::Color, hittable::Hit, ray::Ray};
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
#[derive(Clone)]
pub enum Materials {
    Metal(Arc<Metal>),
    Lambertian(Arc<Lambertian>),
    Dielectric(Arc<Dielectric>),
}
pub trait Material: Into<Materials> {
    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

impl Material for Materials {
    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        match self {
            Materials::Metal(m) => m.scatter(r, hit),
            Materials::Lambertian(l) => l.scatter(r, hit),
            Materials::Dielectric(d) => d.scatter(r, hit),
        }
    }
}
impl From<Metal> for Materials {
    fn from(value: Metal) -> Self {
        Materials::Metal(Arc::new(value))
    }
}
impl From<Lambertian> for Materials {
    fn from(value: Lambertian) -> Self {
        Materials::Lambertian(Arc::new(value))
    }
}
impl From<Dielectric> for Materials {
    fn from(value: Dielectric) -> Self {
        Materials::Dielectric(Arc::new(value))
    }
}
