use crate::{
    color::Color,
    ray::Ray,
    shapes::Shape,
    vector::{Point3D, Vec3},
};
use indicatif::ProgressIterator;
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    rngs::ThreadRng,
    thread_rng, Rng,
};
use std::{
    f64::consts::PI,
    io::{self, Write},
    iter::IntoIterator,
};
pub struct CameraBuilder {
    shapes: Vec<Shape>,
    image_height: usize,
    image_width: usize,
    pos: Point3D,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    samples_per_pixel: usize,
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
            focal_length: 1.0,
            viewport_height: 2.0,
            viewport_width: const { 2.0 * (400.0 / 225.0) },
            samples_per_pixel: 100,
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

    pub const fn set_focal_length(mut self, focal_length: f64) -> Self {
        self.focal_length = focal_length;
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
                self.focal_length,
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
}
#[derive(Clone)]
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
}
impl CameraInfo {
    fn new(
        image_height: usize,
        image_width: usize,
        camera_center: Point3D,
        focal_length: f64,
        viewport_height: f64,
        viewport_width: f64,
    ) -> Self {
        eprintln!("viewport_width:{viewport_width},viewport_height:{viewport_height}");
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, focal_length)
            - &viewport_u * 0.5
            - &viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + &(pixel_delta_u + pixel_delta_v) * 0.5;
        Self {
            focal_length,
            viewport_height,
            viewport_width,
            camera_center,
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
    pub fn render<F: Fn(Ray, isize, &[Shape]) -> Vec3>(&mut self, func: F) {
        let stdout = io::stdout();
        let mut stdout_handle = stdout.lock();
        write!(
            stdout_handle,
            "P3\n{} {}\n255\n",
            self.viewport.image_width, self.viewport.image_height
        )
        .expect("can not write to stdout");
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        let mut rng = Uniform::new(0.0, 1.0).sample_iter(thread_rng());
        eprintln!("pix/samp scale:{}", pixel_samples_scale);
        for h in (0..self.viewport.image_height).progress() {
            for w in 0..self.viewport.image_width {
                let mut pixel_color = Vec3::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_sample_ray(w, h, &mut rng);
                    let f = func(r, 50, &self.world);
                    pixel_color += f;
                }
                let pixel_color = Color::new(pixel_color * pixel_samples_scale);
                writeln!(stdout_handle, "{}", pixel_color).expect("can not write to stdout");
            }
        }
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
}
pub fn get_sample_ray(rng: &mut DistIter<Uniform<f64>, ThreadRng, f64>) -> Vec3 {
    Vec3::new(
        rng.next().unwrap_or_default() - 0.5,
        rng.next().unwrap_or_default() - 0.5,
        0.0,
    )
}
