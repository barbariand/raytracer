use crate::{
    color::Color,
    hittable::Hittable,
    materials::Material,
    ray::Ray,
    shapes::Shape,
    vector::{Point3D, Vec3},
};
use indicatif::ParallelProgressIterator;
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    rngs::ThreadRng,
    thread_rng,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    io::{self, Write},
    iter::IntoIterator,
};
pub struct CameraBuilder {
    shapes: Vec<Shape>,
    image_height: usize,
    image_width: usize,
    pos: Point3D,
    viewport_height: f64,
    viewport_width: f64,
    samples_per_pixel: usize,
    vfov: f64,
    look_at: Vec3,
}
impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl CameraBuilder {
    pub const fn new() -> Self {
        Self {
            shapes: Vec::new(),
            image_height: 225,
            image_width: 400,
            pos: Point3D::new(0., 0., 0.),
            look_at: Vec3::new(0.0, 0.0, 1.0),
            viewport_height: 2.0,
            viewport_width: const { 2.0 * (400.0 / 225.0) * 90.0 },
            samples_per_pixel: 100,
            vfov: 90.0,
        }
    }
    pub fn add_shape(mut self, s: impl Into<Shape>) -> Self {
        self.shapes.push(s.into());
        self
    }
    pub fn add_shapes(mut self, s: impl IntoIterator<Item = impl Into<Shape>>) -> Self {
        self.shapes.extend(s.into_iter().map(Into::into));
        self
    }

    pub fn set_image_height(mut self, window_height: usize) -> Self {
        self.image_height = window_height;
        self.viewport_width = self.viewport_height * self.aspect_ratio();
        self
    }

    pub fn set_image_height_with_aspect_ratio(
        mut self,
        window_height: usize,
        aspect_ratio: f64,
    ) -> Self {
        self.image_height = window_height;
        self.image_width = (window_height as f64 * (aspect_ratio)) as usize;
        self.viewport_width = self.viewport_height * aspect_ratio;
        self
    }

    pub fn set_image_width(mut self, image_width: usize) -> Self {
        self.image_width = image_width;
        self.viewport_height = self.viewport_width / self.aspect_ratio();
        self
    }

    pub fn set_image_width_with_aspect_ratio(
        mut self,
        window_width: usize,
        aspect_ratio: f64,
    ) -> Self {
        self.image_width = window_width;
        self.image_height = (window_width as f64 * (1.0 / aspect_ratio)) as usize;
        self.viewport_height = self.viewport_width / self.aspect_ratio();
        self
    }

    pub const fn set_pos(mut self, pos: Point3D) -> Self {
        self.pos = pos;
        self
    }
    pub fn aspect_ratio(&self) -> f64 {
        self.image_height as f64 / self.image_width as f64
    }

    pub fn build(self) -> Camera {
        Camera {
            viewport: CameraInfo::new(
                self.image_height,
                self.image_width,
                self.pos,
                self.look_at,
                self.viewport_height,
                self.viewport_width,
            ),
            world: self.shapes,
            samples_per_pixel: self.samples_per_pixel,
        }
    }

    pub fn set_samples_per_pixel(&mut self, samples_per_pixel: usize) -> &mut Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn set_vfov(&mut self, vfov_deg: f64) -> &mut Self {
        let new_vfov = (vfov_deg.to_degrees() * 0.5).tan();
        self.viewport_height = self.viewport_height / self.vfov * new_vfov;
        self.viewport_width = self.viewport_width / self.vfov * new_vfov;
        self.vfov = new_vfov;
        self
    }
}
#[derive(Clone, Copy)]
pub struct CameraInfo {
    image_height: usize,
    image_width: usize,
    camera_center: Point3D,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    viewport_upper_left: Vec3,
    look_at: Vec3,
    vup: Vec3,
    u_base: Vec3,
    v_base: Vec3,
    w_base: Vec3,
}
impl CameraInfo {
    fn new(
        image_height: usize,
        image_width: usize,
        look_from: Vec3,
        look_at: Vec3,
        viewport_height: f64,
        viewport_width: f64,
    ) -> Self {
        let focal_length = (look_from - look_at).length();

        let vup = Vec3::new(0.0, 1.0, 0.0);
        let w_base = (look_from - look_at).unit_vector();
        let u_base = vup.cross(&w_base).unit_vector();
        let v_base = w_base.cross(&u_base);

        let viewport_u = viewport_width * u_base;
        let viewport_v = viewport_height * -v_base;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            look_from - focal_length * w_base - &viewport_u * 0.5 - &viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + &(pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            u_base,
            v_base,
            vup,
            w_base,
            focal_length,
            viewport_height,
            viewport_width,
            camera_center: look_from,
            look_at,
            image_height,
            image_width,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel00_loc,
            pixel_delta_v,
            viewport_upper_left,
        }
    }

    pub const fn window_height(&self) -> usize {
        self.image_height
    }

    pub const fn window_width(&self) -> usize {
        self.image_width
    }

    pub const fn camera_center(&self) -> Point3D {
        self.camera_center
    }

    pub const fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub const fn viewport_width(&self) -> f64 {
        self.viewport_width
    }

    pub const fn viewport_height(&self) -> f64 {
        self.viewport_height
    }

    pub const fn viewport_u(&self) -> &Vec3 {
        &self.viewport_u
    }

    pub const fn viewport_v(&self) -> &Vec3 {
        &self.viewport_v
    }

    pub const fn pixel_delta_u(&self) -> &Vec3 {
        &self.pixel_delta_u
    }

    pub const fn pixel_delta_v(&self) -> &Vec3 {
        &self.pixel_delta_v
    }

    pub const fn pixel00_loc(&self) -> &Vec3 {
        &self.pixel00_loc
    }

    pub const fn viewport_upper_left(&self) -> &Vec3 {
        &self.viewport_upper_left
    }

    pub fn set_camera_center(&mut self, camera_center: Point3D) {
        self.camera_center = camera_center;
        self.recalculate()
    }

    pub fn set_look_at(&mut self, look_at: Vec3) {
        self.look_at = look_at;
        self.recalculate();
    }
    pub fn recalculate(&mut self) {
        self.focal_length = (self.camera_center - self.look_at).length();

        self.w_base = (self.camera_center - self.look_at).unit_vector();
        self.u_base = self.vup.cross(&self.w_base).unit_vector();
        self.v_base = self.w_base.cross(&self.u_base);

        self.viewport_u = self.viewport_width * self.u_base;
        self.viewport_v = self.viewport_height * -self.v_base;

        self.pixel_delta_u = self.viewport_u / self.image_width as f64;
        self.pixel_delta_v = self.viewport_v / self.image_height as f64;

        self.viewport_upper_left = self.camera_center
            - self.focal_length * self.w_base
            - &self.viewport_u * 0.5
            - &self.viewport_v * 0.5;
        self.pixel00_loc =
            self.viewport_upper_left + &(self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }
}

#[derive(Clone)]
pub struct Camera {
    viewport: CameraInfo,
    world: Vec<Shape>,
    samples_per_pixel: usize,
}
impl Camera {
    pub const fn area(&self) -> usize {
        self.viewport.image_width * self.viewport.image_height
    }
    pub fn render(&mut self) {
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        eprintln!("pix/samp scale:{}", pixel_samples_scale);
        let s = (0..self.viewport.image_height)
            .into_par_iter()
            .progress()
            .map(|h| {
                let mut rng = Uniform::new(0.0, 1.0).sample_iter(thread_rng());
                (0..self.viewport.image_width).fold(String::new(), |s, w| {
                    let mut pixel_color = Vec3::new(0., 0., 0.);
                    for _ in 0..self.samples_per_pixel {
                        let r: Ray = self.get_sample_ray(w, h, &mut rng);
                        let f = ray_color(r, 50, &self.world);
                        pixel_color += f;
                    }
                    let pixel_color = Color::new(pixel_color * pixel_samples_scale);
                    s + &format!("{}\n", pixel_color)
                })
            })
            .reduce(String::new, |acc, v| acc + &v);
        let stdout = io::stdout();
        let mut stdout_handle = stdout.lock();
        write!(
            stdout_handle,
            "P3\n{} {}\n255\n",
            self.viewport.image_width, self.viewport.image_height
        )
        .expect("can not write to stdout");
        eprintln!("writing to stdout");
        println!("{}", s);
        eprintln!("Done");
    }
    pub fn get_sample_ray(
        &self,
        w: usize,
        h: usize,
        rng: &mut DistIter<Uniform<f64>, ThreadRng, f64>,
    ) -> Ray {
        let offset = get_sample_ray(rng);
        let pixel_center = self.viewport.pixel00_loc()
            + &((w as f64 + offset.x()) * self.viewport.pixel_delta_u())
            + ((h as f64 + offset.y()) * self.viewport.pixel_delta_v());
        let ray_direction = pixel_center - self.viewport.camera_center();

        Ray::new(self.viewport.camera_center(), ray_direction)
    }

    pub fn set_camera_center(&mut self, camera_center: Point3D) {
        self.viewport.set_camera_center(camera_center)
    }

    pub fn set_look_at(&mut self, look_at: Vec3) {
        self.viewport.set_look_at(look_at)
    }
}
pub fn get_sample_ray(rng: &mut DistIter<Uniform<f64>, ThreadRng, f64>) -> Vec3 {
    Vec3::new(
        rng.next().unwrap_or_default() - 0.5,
        rng.next().unwrap_or_default() - 0.5,
        0.0,
    )
}
fn ray_color(r: Ray, depth: isize, hittable: &[Shape]) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }
    if let Some(hit) = hittable.hit(&r) {
        if let Some((scatterd, color)) = hit.mat.scatter(&r, &hit) {
            return color.vec3() * &ray_color(scatterd, depth - 1, hittable);
        }
        return Vec3::ZERO;
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}
