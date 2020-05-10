use crate::camera::Camera;
use crate::materials::Material::*;
use crate::objects::Sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::rngs::ThreadRng;
use rand::Rng;

pub mod camera;
pub mod hit;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod vec3;

pub fn draw(mut rng: &mut ThreadRng, nx: i32, ny: i32, ns: i32, world: &[Sphere]) -> String {
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        &look_from,
        &look_at,
        &Vec3(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );

    let mut image_content = String::new();
    image_content.push_str(format!("P3\n{} {}\n255\n", nx, ny).as_str());
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (i as f32 + rng.gen_range(0.0, 1.0)) / nx as f32;
                let v = (j as f32 + rng.gen_range(0.0, 1.0)) / ny as f32;
                let ray = camera.get_ray(u, v, &mut rng);
                // let _point = ray.calc_point(2.0);
                col += color(&ray, world, 0, &mut rng);
            }
            col /= ns as f32;
            col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());

            let r = (col.0 * 255.99) as i32;
            let g = (col.1 * 255.99) as i32;
            let b = (col.2 * 255.99) as i32;
            image_content.push_str(format!("{} {} {}\n", r, g, b).as_str());
        }
    }
    image_content
}

fn color(r: &Ray, world: &[Sphere], depth: i32, rng: &mut ThreadRng) -> Vec3 {
    if let Some(hit_record) = objects::hit(world, r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(r, &hit_record, rng)
            {
                return attenuation * color(&scattered, world, depth + 1, rng);
            }
        }
        return Vec3::default();
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.1 + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

pub fn random_scene(rng: &mut ThreadRng) -> Vec<Sphere> {
    let mut hitable_list: Vec<Sphere> = Vec::new();
    hitable_list.push(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian {
            albedo: Vec3(0.5, 0.5, 0.5),
        },
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Vec3(
                a as f32 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as f32 + 0.9 * rng.gen_range(0.0, 1.0),
            );
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    hitable_list.push(Sphere::new(
                        center,
                        0.2,
                        Lambertian {
                            albedo: Vec3(
                                rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                                rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                                rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0),
                            ),
                        },
                    ));
                } else if choose_mat < 0.95 {
                    hitable_list.push(Sphere::new(
                        center,
                        0.2,
                        Metal {
                            albedo: Vec3(
                                rng.gen_range(0.5, 1.0),
                                rng.gen_range(0.5, 1.0),
                                rng.gen_range(0.5, 1.0),
                            ),
                            fuzz: rng.gen_range(0.0, 0.5),
                        },
                    ));
                } else {
                    hitable_list.push(Sphere::new(center, 0.2, Dielectric { ref_idx: 1.5 }));
                }
            }
        }
    }

    hitable_list.push(Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        Dielectric { ref_idx: 1.5 },
    ));
    hitable_list.push(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1),
        },
    ));
    hitable_list.push(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    ));

    hitable_list
}
