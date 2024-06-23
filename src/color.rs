use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use crate::vector::Vec3;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Color(Vec3);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            " {} {} {}",
            (255.999 * self.red()) as u8,
            (255.999 * self.green()) as u8,
            (255.999 * self.blue()) as u8
        )
    }
}
impl Color {
    pub const WHITE: Color = Color(Vec3::new(1.0, 1.0, 1.0));
    pub const RED: Color = Color(Vec3::new(1.0, 0.0, 0.0));
    pub const GREEN: Color = Color(Vec3::new(0.0, 1.0, 0.0));
    pub const BLUE: Color = Color(Vec3::new(0.0, 0.0, 1.0));
    pub fn from_f64s(red: f64, green: f64, blue: f64) -> Color {
        assert!(
            (0.0..=1.0).contains(&red),
            "red is out of range 0-1 value is:{}",
            red
        );
        assert!(
            (0.0..=1.0).contains(&green),
            "green is out of range 0-1 value is:{}",
            green
        );
        assert!(
            (0.0..=1.0).contains(&blue),
            "blue is out of range 0-1 value is:{}",
            blue
        );
        Self(Vec3::new(red, green, blue))
    }
    pub fn new(v: Vec3) -> Self {
        let s = Self(v);
        s.validate_or_crash();
        s
    }
    pub const fn red(&self) -> &f64 {
        self.0.x()
    }
    pub const fn green(&self) -> &f64 {
        self.0.y()
    }
    pub const fn blue(&self) -> &f64 {
        self.0.z()
    }
    pub fn red_mut(&mut self) -> &mut f64 {
        self.0.x_mut()
    }
    pub fn green_mut(&mut self) -> &mut f64 {
        self.0.y_mut()
    }
    pub fn blue_mut(&mut self) -> &mut f64 {
        self.0.z_mut()
    }
    pub fn validate_or_crash(&self) {
        assert!(
            (0.0..=1.0).contains(self.red()),
            "red is out of range 0-1 value is:{}",
            self.red()
        );
        assert!(
            (0.0..=1.0).contains(self.green()),
            "green is out of range 0-1 value is:{}",
            self.green()
        );
        assert!(
            (0.0..=1.0).contains(self.blue()),
            "blue is out of range 0-1 value is:{}",
            self.blue()
        );
    }
    pub fn maybe_new(v: Vec3) -> Option<Self> {
        if !((0.0..=1.0).contains(v.x())) {
            eprintln!("red is out of range");
            return None;
        }
        if !((0.0..=1.0).contains(v.y())) {
            eprintln!("green is out of range, green is:{}", v.y());
            return None;
        }
        if !((0.0..=1.0).contains(v.z())) {
            eprintln!("blue is out of range");
            return None;
        }
        Some(Color(v))
    }
}
impl Neg for &Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        Color::from_f64s(-self.red(), -self.green(), -self.blue())
    }
}
impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::from_f64s(
            self.red() + rhs.red(),
            self.green() + rhs.green(),
            self.blue() + rhs.blue(),
        )
    }
}
impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::from_f64s(
            self.red() - rhs.red(),
            self.green() - rhs.green(),
            self.blue() - rhs.blue(),
        )
    }
}
impl Sub for &Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::from_f64s(
            self.red() - rhs.red(),
            self.green() - rhs.green(),
            self.blue() - rhs.blue(),
        )
    }
}
impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::from_f64s(
            self.red() + rhs.red(),
            self.green() + rhs.green(),
            self.blue() + rhs.blue(),
        )
    }
}
impl Mul for &Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::from_f64s(
            self.red() * rhs.red(),
            self.green() * rhs.green(),
            self.blue() * rhs.blue(),
        )
    }
}
impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::from_f64s(self.red() * rhs, self.green() * rhs, self.blue() * rhs)
    }
}
impl Mul<&Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}
impl Mul<usize> for &Color {
    type Output = Color;
    fn mul(self, rhs: usize) -> Self::Output {
        let rhs = rhs as f64;
        Color::from_f64s(self.red() * rhs, self.green() * rhs, self.blue() * rhs)
    }
}
impl Mul<&Color> for usize {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}
impl Div<f64> for &Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl MulAssign<&f64> for Color {
    fn mul_assign(&mut self, rhs: &f64) {
        self.0 *= rhs
    }
}
impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self.red_mut() /= rhs;
        *self.green_mut() /= rhs;
        *self.blue_mut() /= rhs;
    }
}
impl AddAssign<&f64> for Color {
    fn add_assign(&mut self, rhs: &f64) {
        *self.red_mut() *= *rhs;
        *self.green_mut() *= *rhs;
        *self.blue_mut() *= *rhs;
    }
}
