use crate::{color::Color, ray::Ray, vector::random_unit_vector};

use super::Material;

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _: &crate::ray::Ray, hit: &crate::hittable::Hit) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scatterd = Ray::new(hit.p, scatter_direction);
        Some((scatterd, self.color))
    }
}
