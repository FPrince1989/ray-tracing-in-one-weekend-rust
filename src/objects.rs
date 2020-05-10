use crate::hit::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Sphere {
    /// dot(r.dir,r.dir)*t*t + 2*dot(r.dir, r.origin-center)*t + dot(r.origin-center-r.origin-center) - radius*radius = 0
    /// -b-sqrt(b*b - 4*a*c) / 2*a
    /// b' = b/2
    /// -2*b' - 2*sqrt(b'*b' - a*c) / 2*a
    /// -b' - sqrt(b'*b' -a*c) /a
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        macro_rules! check_return {
            ($t:expr) => {
                if $t < t_max && $t > t_min {
                    let p = ray.calc_point($t);
                    let normal = (p - self.center) / self.radius;
                    return Some(HitRecord {
                        t: $t,
                        p,
                        normal,
                        material: self.material,
                    });
                }
            };
        }

        let oc = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let b = Vec3::dot(&oc, &ray.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant >= 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            check_return!(t);
            let t = (-b + discriminant.sqrt()) / a;
            check_return!(t);
        }

        None
    }
}

pub fn hit(sphere_vec: &[Sphere], r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut closet = t_max;
    let mut result = None;
    for sphere in sphere_vec.iter() {
        if let Some(hit_record) = sphere.hit(r, t_min, closet) {
            closet = hit_record.t;
            result = Some(hit_record);
        }
    }

    result
}
