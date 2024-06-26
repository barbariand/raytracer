use crate::{color::Color, ray::Ray, vector::random_unit_in_disk};

use super::Material;
#[derive(Debug, Clone)]
pub struct Metal {
    color: Color,
    fuzz: f64,
}

impl Metal {
    pub const fn new(color: Color, fuzz: f64) -> Self {
        Self { color, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, r: &crate::ray::Ray, hit: &crate::hittable::Hit) -> Option<(Ray, Color)> {
        let reflected = r.direction().reflect(&hit.normal);
        let reflected = reflected.unit_vector() + self.fuzz * random_unit_in_disk();
        if reflected.dot(&hit.normal) <= 0.0 {
            return None;
        }
        Some((Ray::new(hit.p, reflected, r.tm()), self.color))
    }
}
