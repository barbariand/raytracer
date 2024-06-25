use crate::{
    materials::Materials,
    ray::Ray,
    vector::{Point3D, Vec3},
};
pub struct Hit {
    pub p: Point3D,
    pub normal: Vec3,
    pub mat: Materials,
    pub front_face: bool,
}
impl Hit {
    pub fn new(r: &Ray, p: Point3D, normal: Vec3, mat: Materials) -> Self {
        //assert!(normal==normal.unit_vector());
        let front_face = r.direction().dot(&normal) < 0.0;
        let normal = match front_face {
            true => normal,
            false => -normal,
        };
        Self {
            mat,
            p,
            normal,
            front_face,
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray) -> Option<Hit>;
}
