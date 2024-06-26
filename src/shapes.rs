mod sphere;
pub use sphere::Sphere;

pub use crate::hittable::*;
#[derive(Clone, Debug)]
pub enum Shape {
    Sphere(Sphere),
}
impl From<Sphere> for Shape {
    fn from(value: Sphere) -> Self {
        Self::Sphere(value)
    }
}
impl Hittable for Shape {
    fn hit(&self, r: &crate::ray::Ray) -> Option<Hit> {
        match self {
            Shape::Sphere(s) => s.hit(r),
        }
    }
}
