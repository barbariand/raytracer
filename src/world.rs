use crate::{hittable::Hittable, shapes::Shape};

pub struct World {
    shapes: Vec<Shape>,
}
impl World {
    pub const fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, s: impl Into<Shape>) {
        self.shapes.push(s.into());
    }
    pub fn add_shapes(&mut self, s: impl IntoIterator<Item = impl Into<Shape>>) {
        self.shapes.extend(s.into_iter().map(Into::into));
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> Hittable for SharedWorld<'a> {
    fn hit(&self, r: &crate::ray::Ray) -> Option<crate::hittable::Hit> {
        self.shapes
            .iter()
            .flat_map(|s| s.hit(r))
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }
}
impl<'a> From<&'a World> for SharedWorld<'a> {
    fn from(value: &'a World) -> Self {
        SharedWorld {
            shapes: &value.shapes,
        }
    }
}

pub struct SharedWorld<'a> {
    pub shapes: &'a [Shape],
}
