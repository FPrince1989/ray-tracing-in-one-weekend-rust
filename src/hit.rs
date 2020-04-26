use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}


pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}