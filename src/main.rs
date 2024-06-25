#![warn(clippy::missing_const_for_fn, clippy::perf)]
use camera::CameraBuilder;
use color::Color;
use materials::{Dielectric, Lambertian, Materials, Metal};

use shapes::Sphere;
use vector::{Point3D, Vec3};

pub mod camera;
pub mod color;
mod hittable;
mod materials;
mod ray;
mod shapes;
mod vector;

fn main() {
    let material_ground: Materials = Lambertian::new(Color::new(Vec3::new(0.8, 0.8, 0.0))).into();
    let material_center: Materials = Lambertian::new(Color::new(Vec3::new(0.1, 0.2, 0.5))).into();
    let material_left: Materials = Dielectric::new(1.5).into();
    let material_bubble: Materials = Dielectric::new(1.0 / 1.50).into();
    let material_right: Materials = Metal::new(Color::new(Vec3::new(0.8, 0.6, 0.2)), 1.0).into();

    let s = [
        Sphere::new(
            Point3D::new(0.0, -100.5, -1.0),
            100.0,
            material_ground.clone(),
        ),
        Sphere::new(Point3D::new(0.0, 0.0, -1.2), 0.5, material_center.clone()),
        Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, material_left.clone()),
        Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.4, material_bubble),
        Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, material_right.clone()),
    ];

    let mut camera = CameraBuilder::new()
        .set_image_height_with_aspect_ratio(720, 16.0 / 9.0)
        .add_shapes(s)
        .build();
    camera.set_camera_center(Point3D::new(-2.0, -2.0, 1.0));
    camera.set_look_at(Vec3::new(0.0, 1.0, 0.0));
    camera.render();
}
