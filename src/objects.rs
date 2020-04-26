use std::rc::Rc;

use crate::hit::{Hitable, HitRecord, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    /// dot(r.dir,r.dir)*t*t + 2*dot(r.dir, r.origin-center)*t + dot(r.origin-center-r.origin-center) - radius*radius = 0
    /// -b-sqrt(b*b - 4*a*c) / 2*a
    /// b' = b/2
    /// -2*b' - 2*sqrt(b'*b' - a*c) / 2*a
    /// -b' - sqrt(b'*b' -a*c) /a
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant >= 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.calc_point(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: self.material.clone(),
                });
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.calc_point(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: self.material.clone(),
                });
            }
        }

        None
    }
}


pub struct HitableList {
    list: Vec<Rc<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Rc<dyn Hitable>>) -> HitableList {
        HitableList {
            list,
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closet = t_max;
        let mut result = None;
        for hitable in self.list.iter() {
            if let Some(hit_record) = hitable.hit(r, t_min, closet) {
                closet = hit_record.t;
                result = Some(hit_record);
            }
        }

        result
    }
}