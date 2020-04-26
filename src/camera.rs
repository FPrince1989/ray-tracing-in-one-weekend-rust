use std::f32::consts::PI;

use rand::{Rng, thread_rng};

use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Default, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, v_up: &Vec3, v_fov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let lens_radius = aperture / 2.0;
        let origin = *look_from;
        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(v_up, &w).unit_vector();
        let v = Vec3::cross(&w, &u);
        let lower_left_corner = origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * half_width * focus_dist * 2.0;
        let vertical = v * half_height * focus_dist * 2.0;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let result = Vec3(thread_rng().gen_range(-1.0, 1.0), thread_rng().gen_range(-1.0, 1.0), 0.0);
        if Vec3::dot(&result, &result) < 1.0 {
            return result;
        }
    }
}