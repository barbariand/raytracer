#![warn(clippy::missing_const_for_fn)]
#![feature(iter_array_chunks)]

use camera::CameraBuilder;
use ray::Ray;
use shapes::{Hittable, Shape, Sphere};
use vector::{random_unit_vector, random_unit_vector_in_hemisphere, Point3D, Vec3};

pub mod camera;
pub mod color;
mod hittable;
mod ray;
mod shapes;
mod vector;

fn ray_color(r: Ray, depth: isize, hittable: &[Shape]) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }
    if let Some(hit) = hittable.hit(&r) {
        let direction = hit.normal + random_unit_vector();
        return 0.5 * ray_color(Ray::new(hit.p, direction), depth - 1, hittable);
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}
fn main() {
    let s = [
        Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0),
    ];

    let mut camera = CameraBuilder::new().add_shapes(s).build();
    camera.render(ray_color);
}
