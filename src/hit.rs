use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::ThreadRng;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait MaterialTrait {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, rng: &mut ThreadRng)
        -> Option<(Vec3, Ray)>;
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
