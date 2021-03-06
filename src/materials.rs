use rand::Rng;

use crate::hit::{HitRecord, MaterialTrait};
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::ThreadRng;

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let result = Vec3(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        if result.squared_length() <= 1.0 {
            return result;
        }
    }
}

impl Material {
    #[inline]
    pub fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian(m) => m.scatter(ray_in, record, rng),
            Material::Metal(m) => m.scatter(ray_in, record, rng),
            Material::Dielectric(m) => m.scatter(ray_in, record, rng),
        }
    }
}

#[inline]
fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    v - &(normal * Vec3::dot(v, normal) * 2.0)
}

#[inline]
fn refract(v: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let v = v.unit_vector();
    let dt = Vec3::dot(&v, &normal);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        Some((v - normal * dt) * ni_over_nt - normal * discriminant.sqrt())
    } else {
        None
    }
}

#[inline]
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl MaterialTrait for Lambertian {
    #[inline]
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let target = record.p + record.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(record.p, target - record.p);
        Some((self.albedo, scattered))
    }
}

impl MaterialTrait for Metal {
    #[inline]
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflected = reflect(&ray_in.direction.unit_vector(), &record.normal);
        let scattered = Ray::new(record.p, reflected + random_in_unit_sphere(rng) * fuzz);
        if Vec3::dot(&scattered.direction, &record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl MaterialTrait for Dielectric {
    #[inline]
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3(1.0, 1.0, 1.0);
        let reflected = reflect(&ray_in.direction, &record.normal);
        let (outward_normal, ni_over_nt, cosine) =
            if Vec3::dot(&ray_in.direction, &record.normal) > 0.0 {
                (
                    record.normal * -1.0,
                    self.ref_idx,
                    self.ref_idx * Vec3::dot(&ray_in.direction, &record.normal)
                        / ray_in.direction.length(),
                )
            } else {
                (
                    record.normal,
                    1.0 / self.ref_idx,
                    -Vec3::dot(&ray_in.direction, &record.normal) / ray_in.direction.length(),
                )
            };

        let refracted_opt = refract(&ray_in.direction, &outward_normal, ni_over_nt);
        let reflect_prob = if refracted_opt.is_some() {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };
        let ray_out = if rng.gen_range(0.0, 1.0) < reflect_prob {
            Ray::new(record.p, reflected)
        } else {
            Ray::new(record.p, refracted_opt.unwrap())
        };
        Some((attenuation, ray_out))
    }
}
