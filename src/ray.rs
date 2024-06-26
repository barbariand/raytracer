use crate::vector::{Point3D, Vec3};

pub struct Ray {
    origin: Point3D,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub const fn new(origin: Point3D, dir: Vec3, tm: f64) -> Self {
        Self { origin, dir, tm }
    }

    pub const fn origin(&self) -> &Point3D {
        &self.origin
    }
    pub const fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }

    pub const fn tm(&self) -> f64 {
        self.tm
    }
}
