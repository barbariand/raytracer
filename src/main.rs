#![warn(clippy::missing_const_for_fn, clippy::perf)]
use camera::Camera;
use color::Color;
use materials::{Dielectric, Lambertian, Materials, Metal};

use rand::{
    distributions::{DistIter, Distribution, Uniform},
    rngs::ThreadRng,
    thread_rng,
};
use shapes::Sphere;
use vector::{Point3D, Vec3};
use world::World;

pub mod camera;
pub mod color;
mod hittable;
mod materials;
mod ray;
mod shapes;
mod vector;
pub mod world;

fn main() {
    let mut world = World::new();
    let ground_material: Materials = Lambertian::new(Color::new(Vec3::new(0.5, 0.5, 0.5))).into();
    world.add_shape(Sphere::new(
        Point3D::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut between = Uniform::new(0.0, 1.0).sample_iter(thread_rng());

    let mut shapes = Vec::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = between.next().unwrap_or_default();
            let center = Point3D::new(
                a as f64 + 0.9 * between.next().unwrap_or_default(),
                0.2,
                b as f64 + 0.9 * between.next().unwrap_or_default(),
            );

            if (center - Point3D::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Materials = if choose_mat < 0.8 {
                    let color = random_color(&mut between) * random_color(&mut between);
                    Lambertian::new(color).into()
                } else if choose_mat < 0.95 {
                    let color = &random_color(&mut between) * 0.5 + 0.5_f64 * &Color::WHITE;
                    let fuzz = between.next().unwrap_or_default() * 0.5;
                    Metal::new(color, fuzz).into()
                } else {
                    Dielectric::new(1.5).into()
                };

                shapes.push(Sphere::new(center, 0.2, material));
            }
        }
    }
    world.add_shapes(shapes);
    let material = Dielectric::new(1.5).into();
    world.add_shape(Sphere::new(Point3D::new(0.0, 1.0, 0.0), 1.0, material));

    let material = Lambertian::new(Color::new(Vec3::new(0.4, 0.2, 0.1))).into();
    world.add_shape(Sphere::new(Point3D::new(-4.0, 1.0, 0.0), 1.0, material));

    let material = Metal::new(Color::new(Vec3::new(0.7, 0.6, 0.5)), 0.0).into();
    world.add_shape(Sphere::new(Point3D::new(4.0, 1.0, 0.0), 1.0, material));

    eprintln!("Setup done");
    let camera = Camera::default()
        .set_image_height_with_aspect_ratio(720, 16.0 / 9.0)
        .set_samples_per_pixel(500)
        .set_vfov(20.0)
        .set_camera_center(Point3D::new(13.0, 2.0, 3.0))
        .set_look_at(Vec3::new(0.0, 0.0, 0.0))
        .set_defocus_angle(0.6)
        .set_focus_dist(10.0);
    camera.render(&world);
}
fn random_color(between: &mut DistIter<Uniform<f64>, ThreadRng, f64>) -> Color {
    Color::new(Vec3::new(
        between.next().unwrap_or_default(),
        between.next().unwrap_or_default(),
        between.next().unwrap_or_default(),
    ))
}
