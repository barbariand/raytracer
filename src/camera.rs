use crate::{
    color::Color,
    hittable::Hittable,
    materials::Material,
    ray::Ray,
    vector::{random_unit_in_disk, Point3D, Vec3},
    world::{SharedWorld, World},
};
use indicatif::ParallelProgressIterator;
use rand::{
    distributions::{DistIter, Distribution, Uniform},
    rngs::ThreadRng,
    thread_rng,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::io::{self, Write};

impl Default for CameraInfo {
    fn default() -> Self {
        Self::private_new(
            225,
            400,
            Point3D::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            90.0,
            10.0,
            0.0,
        )
    }
}
#[derive(Clone, Copy)]
pub struct CameraInfo {
    image_height: usize,
    image_width: usize,
    camera_center: Point3D,
    vfov: f64,
    defocus_angle: f64,
    focus_dist: f64,
    ///autocalculated values from aboce
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
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}
impl CameraInfo {
    fn private_new(
        image_height: usize,
        image_width: usize,
        look_from: Vec3,
        look_at: Vec3,
        vfov: f64,
        focus_dist: f64,
        defocus_angle: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let w_base = (look_from - look_at).unit_vector();
        let u_base = vup.cross(&w_base).unit_vector();
        let v_base = w_base.cross(&u_base);

        let viewport_u = viewport_width * u_base;
        let viewport_v = viewport_height * -v_base;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            look_from - focus_dist * w_base - &viewport_u * 0.5 - &viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + &(pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u_base * defocus_radius;
        let defocus_disk_v = v_base * defocus_radius;
        Self {
            vfov,
            u_base,
            v_base,
            vup,
            w_base,
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
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
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
    }

    pub fn set_look_at(&mut self, look_at: Vec3) {
        self.look_at = look_at;
    }
    pub fn recalculate(&mut self) {
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        self.viewport_height = 2.0 * h * self.focus_dist;
        self.viewport_width =
            self.viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w_base = (self.camera_center - self.look_at).unit_vector();
        self.u_base = self.vup.cross(&self.w_base).unit_vector();
        self.v_base = self.w_base.cross(&self.u_base);

        self.viewport_u = self.viewport_width * self.u_base;
        self.viewport_v = self.viewport_height * -self.v_base;

        self.pixel_delta_u = self.viewport_u / self.image_width as f64;
        self.pixel_delta_v = self.viewport_v / self.image_height as f64;

        self.viewport_upper_left = self.camera_center
            - self.focus_dist * self.w_base
            - &self.viewport_u * 0.5
            - &self.viewport_v * 0.5;
        self.pixel00_loc =
            self.viewport_upper_left + &(self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u_base * defocus_radius;
        self.defocus_disk_v = self.v_base * defocus_radius;
    }

    pub fn set_focus_dist(&mut self, focus_dist: f64) {
        self.focus_dist = focus_dist;
    }

    pub fn set_defocus_angle(&mut self, defocus_angle: f64) {
        self.defocus_angle = defocus_angle;
    }

    pub fn set_vfov(&mut self, vfov: f64) {
        self.vfov = vfov;
    }

    pub fn set_image_height(&mut self, image_height: usize) {
        self.image_height = image_height;
    }

    pub fn set_image_width(&mut self, image_width: usize) {
        self.image_width = image_width;
    }
    pub fn set_image_height_with_aspect_ratio(&mut self, window_height: usize, aspect_ratio: f64) {
        self.image_height = window_height;
        self.image_width = (window_height as f64 * (aspect_ratio)) as usize;
        self.viewport_width = self.viewport_height * aspect_ratio;
    }

    pub fn set_image_width_with_aspect_ratio(&mut self, window_width: usize, aspect_ratio: f64) {
        self.image_width = window_width;
        self.image_height = (window_width as f64 * (1.0 / aspect_ratio)) as usize;
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            viewport: Default::default(),
            samples_per_pixel: 20,
        }
    }
}
#[derive(Clone)]
pub struct Camera {
    viewport: CameraInfo,
    samples_per_pixel: usize,
}

impl Camera {
    pub const fn area(&self) -> usize {
        self.viewport.image_width * self.viewport.image_height
    }

    pub fn set_camera_center(mut self, camera_center: Point3D) -> Self {
        self.viewport.set_camera_center(camera_center);
        self.viewport.recalculate();
        self
    }

    pub fn set_look_at(mut self, look_at: Vec3) -> Self {
        self.viewport.set_look_at(look_at);
        self.viewport.recalculate();
        self
    }

    pub fn set_samples_per_pixel(mut self, samples_per_pixel: usize) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self.viewport.recalculate();
        self
    }

    pub fn set_defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.viewport.set_defocus_angle(defocus_angle);
        self.viewport.recalculate();
        self
    }

    pub fn set_focus_dist(mut self, focus_dist: f64) -> Self {
        self.viewport.set_focus_dist(focus_dist);
        self.viewport.recalculate();
        self
    }

    pub fn set_image_height(mut self, image_height: usize) -> Self {
        self.viewport.set_image_height(image_height);
        self.viewport.recalculate();
        self
    }

    pub fn set_image_height_with_aspect_ratio(
        mut self,
        window_height: usize,
        aspect_ratio: f64,
    ) -> Self {
        self.viewport
            .set_image_height_with_aspect_ratio(window_height, aspect_ratio);
        self.viewport.recalculate();
        self
    }

    pub fn set_image_width(&mut self, image_width: usize) -> &mut Self {
        self.viewport.set_image_width(image_width);
        self.viewport.recalculate();
        self
    }

    pub fn set_image_width_with_aspect_ratio(
        mut self,
        window_width: usize,
        aspect_ratio: f64,
    ) -> Self {
        self.viewport
            .set_image_width_with_aspect_ratio(window_width, aspect_ratio);
        self.viewport.recalculate();
        self
    }

    pub fn set_vfov(mut self, vfov: f64) -> Self {
        self.viewport.set_vfov(vfov);
        self.viewport.recalculate();
        self
    }
    pub fn render(&self, world: &World) {
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        eprintln!("pix/samp scale:{}", pixel_samples_scale);
        let shared_world: SharedWorld = world.into();
        let s = (0..self.viewport.image_height)
            .into_par_iter()
            .progress()
            .map(|h| {
                let mut rng = Uniform::new(0.0, 1.0).sample_iter(thread_rng());
                (0..self.viewport.image_width).fold(String::new(), |s, w| {
                    let mut pixel_color = Vec3::new(0., 0., 0.);
                    for _ in 0..self.samples_per_pixel {
                        let r: Ray = self.viewport.get_sample_ray(w, h, &mut rng);
                        let f = ray_color(r, 50, &shared_world);
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
}
impl CameraInfo {
    pub fn get_sample_ray(
        &self,
        w: usize,
        h: usize,
        rng: &mut DistIter<Uniform<f64>, ThreadRng, f64>,
    ) -> Ray {
        let offset = sample_square(rng);
        let pixel_center = self.pixel00_loc()
            + &((w as f64 + offset.x()) * self.pixel_delta_u())
            + ((h as f64 + offset.y()) * self.pixel_delta_v());
        let ray_origin = match self.defocus_angle <= 0.0 {
            true => self.camera_center(),
            false => self.defocus_disk_sample(),
        };
        let ray_direction = pixel_center - ray_origin;

        Ray::new(ray_origin, ray_direction, rng.next().unwrap_or_default())
    }
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_unit_in_disk();
        self.camera_center + (*p.x() * self.defocus_disk_u) + (*p.y() * self.defocus_disk_v)
    }
}
pub fn sample_square(rng: &mut DistIter<Uniform<f64>, ThreadRng, f64>) -> Vec3 {
    Vec3::new(
        rng.next().unwrap_or_default() - 0.5,
        rng.next().unwrap_or_default() - 0.5,
        0.0,
    )
}
fn ray_color<'a>(r: Ray, depth: isize, hittable: &'a SharedWorld<'a>) -> Vec3 {
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
