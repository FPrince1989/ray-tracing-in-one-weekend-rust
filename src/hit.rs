use crate::materials::Material;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}
