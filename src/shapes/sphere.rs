use crate::{
    materials::Materials,
    ray::Ray,
    vector::{Point3D, Vec3},
};

use super::*;
#[derive(Clone, Debug)]
pub struct Sphere {
    center_start: Point3D,
    radius: f64,
    mat: Materials,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, mat: Materials) -> Self {
        Self {
            center_start: center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::ZERO,
        }
    }
    pub fn new_moving(
        center_start: Point3D,
        center_end: Point3D,
        radius: f64,
        mat: Materials,
    ) -> Self {
        Self {
            center_start,
            radius,
            mat,
            is_moving: true,
            center_vec: center_end - center_start,
        }
    }
    pub fn center(&self, time: f64) -> Point3D {
        self.center_start + time * self.center_vec
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let oc = self.center(r.tm()) - *r.origin();
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
        if root_1 < 0.0000001 || root_1.is_nan() {
            root_1 = root_2; // Use the second root if the first is negative
            if root_1 < 0.0000001 || root_1.is_nan() {
                return None; // Both roots are negative, no intersection
            }
        }

        let t = root_1;
        let p = r.at(t);
        let normal = (p - self.center_start) / self.radius;
        Some(Hit::new(r, p, normal, self.mat.clone(), t))
    }
}
