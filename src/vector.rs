use std::{
    f64::consts::PI,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use rand::Rng;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
pub type Point3D = Vec3;
impl Vec3 {
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn x(&self) -> &f64 {
        &self.x
    }

    pub const fn y(&self) -> &f64 {
        &self.y
    }

    pub const fn z(&self) -> &f64 {
        &self.z
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * self.y - rhs.y * self.x,
        )
    }
    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.z
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.y
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.x
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl Mul<usize> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: usize) -> Self::Output {
        let rhs = rhs as f64;
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<&Vec3> for usize {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}
impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl MulAssign<&f64> for Vec3 {
    fn mul_assign(&mut self, rhs: &f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl AddAssign<&f64> for Vec3 {
    fn add_assign(&mut self, rhs: &f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::new(0., 0., 0.), |a, b| a + b)
    }
}
pub fn random_unit_vector_in_hemisphere(normal: &Vec3) -> Vec3 {
    let mut random_vector = random_unit_vector();
    // Align the random vector to the normal vector
    if random_vector.dot(normal) < 0.0 {
        random_vector = -random_vector;
    }

    // Ensure the vector is on the "right" side
    let right_vector = if normal.cross(&random_vector).dot(&Vec3::X) > 0.0 {
        random_vector
    } else {
        -random_vector
    };

    right_vector
}
pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();

    // Generate random spherical coordinates
    let phi = rng.gen_range(0.0..2.0 * PI);
    let theta = rng.gen_range(0.0..PI / 2.0);

    // Convert spherical coordinates to Cartesian coordinates
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();

    // Create the random vector in the unit sphere
    Vec3::new(x, y, z)
}
