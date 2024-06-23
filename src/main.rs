#![warn(clippy::missing_const_for_fn)]

use camera::{CameraBuilder, CameraInfo};
use color::Color;
use ray::Ray;
use shapes::{Hittable, Shape, Sphere};
use vector::{Point3D, Vec3};

pub mod camera;
pub mod color;
mod hittable;
mod ray;
mod shapes;
mod vector;

fn ray_color(r: Ray, hittable: &[Shape]) -> Color {
    if let Some(c) = hittable
        .hit(&r)
        .and_then(|hit| Color::maybe_new(0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0))))
    {
        return c;
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * &Color::from_f64s(1.0, 1.0, 1.0) + a * &Color::from_f64s(0.5, 0.7, 1.0)
}
fn main() {
    let f = |camera: &CameraInfo, world: &[Shape], h: usize, w: usize| -> Color {
        let pixel_center =
            camera.pixel00_loc() + &(w * camera.pixel_delta_u()) + (h * camera.pixel_delta_v());
        let ray_direction = pixel_center - camera.camera_center();

        let r = Ray::new(camera.camera_center(), ray_direction);
        ray_color(r, world)
    };
    let s = [
        Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0),
    ];
    
    let mut camera = CameraBuilder::new().add_shapes(s).build();
    camera.from_function(f);
    println!("{}", camera);
}
