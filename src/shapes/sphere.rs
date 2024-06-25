use crate::{materials::Materials, ray::Ray, vector::Point3D};

use super::*;
#[derive(Clone)]
pub struct Sphere {
    center: Point3D,
    radius: f64,
    mat: Materials,
}

impl Sphere {
    pub const fn new(center: Point3D, radius: f64, mat: Materials) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let oc = &self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root_1 = (h - sqrt_d) / a;
        let mut root_2 = (h + sqrt_d) / a;
        // Ensure root_1 is the smaller root
        if root_1 > root_2 {
            std::mem::swap(&mut root_1, &mut root_2);
        }

        // Check the smallest root first
        if root_1 < 0.0000001 {
            root_1 = root_2; // Use the second root if the first is negative
            if root_1 < 0.0000001 {
                return None; // Both roots are negative, no intersection
            }
        }

        let t = root_1;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        Some(Hit::new(r, p, normal, self.mat.clone()))
    }
}
