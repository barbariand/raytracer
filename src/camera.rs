use crate::{
    color::Color,
    shapes::Shape,
    vector::{Point3D, Vec3},
};
use indicatif::ProgressIterator;
use std::{fmt::Display, iter::IntoIterator, mem::swap};
pub struct CameraBuilder {
    shapes: Vec<Shape>,
    image_height: usize,
    image_width: usize,
    pos: Point3D,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
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
            colors: vec![Color::WHITE; self.image_height * self.image_width],
            viewport: CameraInfo::new(
                self.image_height,
                self.image_width,
                self.pos,
                self.focal_length,
                self.viewport_height,
                self.viewport_width,
            ),
            world: self.shapes,
        }
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
    colors: Vec<Color>,
    viewport: CameraInfo,
    world: Vec<Shape>,
}
impl Camera {
    pub const fn area(&self) -> usize {
        self.viewport.image_width * self.viewport.image_height
    }

    pub fn fill_rest_with(&mut self, color: Color) {
        let area = self.area();
        if self.colors.len() < area {
            let fill = self.colors.capacity() - area;
            self.colors.reserve_exact(fill);
            self.colors.fill(color)
        }
    }
    pub fn from_function<F: Fn(&CameraInfo, &[Shape], usize, usize) -> Color>(&mut self, func: F) {
        let colors = self.colors.iter_mut().enumerate().progress();
        for (i, c) in colors {
            let h = i / self.viewport.image_width;
            let w = i % self.viewport.image_width;
            swap(c, &mut func(&self.viewport, &self.world, h, w));
        }
        eprintln!("Done");
    }
}
impl Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "P3\n{} {}\n255\n",
            self.viewport.image_width, self.viewport.image_height
        )?;
        for c in &self.colors {
            c.fmt(f)?;
        }
        Ok(())
    }
}
