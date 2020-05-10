use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::ThreadRng;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, rng: &mut ThreadRng)
        -> Option<(Vec3, Ray)>;
}
