use std::ops::Neg;

use crate::{
    color::Color,
    ray::Ray,
    vector::{random_f64_in_range, Vec3},
};

use super::Material;
#[derive(Debug, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r: &crate::ray::Ray,
        hit: &crate::hittable::Hit,
    ) -> Option<(crate::ray::Ray, crate::color::Color)> {
        let color = Color::new(Vec3::new(1.0, 1.0, 1.0));
        let ri = match hit.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let unit_direction = r.direction().unit_vector();
        let cos_theta = unit_direction.neg().dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction = match cannot_refract || reflectance(cos_theta, ri) > random_f64_in_range() {
            true => unit_direction.reflect(&hit.normal),
            false => unit_direction.refract(&hit.normal, ri),
        };

        let scatterd = Ray::new(hit.p, direction, r.tm());
        Some((scatterd, color))
    }
}
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
