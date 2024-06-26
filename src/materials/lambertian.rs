use crate::{color::Color, ray::Ray, vector::random_unit_in_disk};

use super::Material;
#[derive(Debug, Clone)]
pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}
impl Material for Lambertian {
    fn scatter(&self, r: &crate::ray::Ray, hit: &crate::hittable::Hit) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + random_unit_in_disk();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scatterd = Ray::new(hit.p, scatter_direction, r.tm());
        Some((scatterd, self.color))
    }
}
