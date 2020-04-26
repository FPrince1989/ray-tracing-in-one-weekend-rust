use rand::{Rng, thread_rng};
use ray_tracing::camera::Camera;
use ray_tracing::hit::Hitable;
use ray_tracing::materials::{Dielectric, Lambertian, Metal};
use ray_tracing::objects::{HitableList, Sphere};
use ray_tracing::ray::Ray;
use ray_tracing::vec3::Vec3;
use std::fs;
use std::rc::Rc;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    println!("Start: {}", start_time);

    let nx = 800;
    let ny = 600;
    let ns = 30;

    let world = random_scene();

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(&look_from, &look_at, &Vec3(0.0, 1.0, 0.0), 20.0, nx as f32 / ny as f32,
                             aperture, dist_to_focus);

    let mut image_content = String::new();
    image_content.push_str(format!("P3\n{} {}\n255\n", nx, ny).as_str());
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (i as f32 + thread_rng().gen_range(0.0, 1.0)) / nx as f32;
                let v = (j as f32 + thread_rng().gen_range(0.0, 1.0)) / ny as f32;
                let ray = camera.get_ray(u, v);
                let _point = ray.calc_point(2.0);
                col += color(&ray, &world, 0);
            }
            col /= ns as f32;
            col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());

            let r = (col.0 * 255.99) as i32;
            let g = (col.1 * 255.99) as i32;
            let b = (col.2 * 255.99) as i32;
            image_content.push_str(format!("{} {} {}\n", r, g, b).as_str());
        }
    }

    fs::write("ray_tracing.ppm", image_content).unwrap();

    let end_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    println!("End: {}", end_time);
    println!("Duration: {}", end_time - start_time);
}


fn color(r: &Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    if let Some(hit_record) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(r, &hit_record) {
                return attenuation * color(&scattered, world, depth + 1);
            }
        }
        return Vec3::default();
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.1 + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn random_scene() -> HitableList {
    let mut hitable_list: Vec<Rc<dyn Hitable>> = Vec::new();
    hitable_list.push(Rc::new(Sphere::new(Vec3(0.0, -1000.0, 0.0), 1000.0,
                                          Rc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5))))));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = thread_rng().gen_range(0.0, 1.0);
            let center = Vec3(
                a as f32 + 0.9 * thread_rng().gen_range(0.0, 1.0),
                0.2,
                b as f32 + 0.9 * thread_rng().gen_range(0.0, 1.0),
            );
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    hitable_list.push(Rc::new(Sphere::new(
                        center, 0.2, Rc::new(Lambertian::new(
                            Vec3(
                                thread_rng().gen_range(0.0, 1.0) * thread_rng().gen_range(0.0, 1.0),
                                thread_rng().gen_range(0.0, 1.0) * thread_rng().gen_range(0.0, 1.0),
                                thread_rng().gen_range(0.0, 1.0) * thread_rng().gen_range(0.0, 1.0),
                            ))))));
                } else if choose_mat < 0.95 {
                    hitable_list.push(Rc::new(Sphere::new(
                        center, 0.2, Rc::new(Metal::new(
                            Vec3(
                                thread_rng().gen_range(0.5, 1.0),
                                thread_rng().gen_range(0.5, 1.0),
                                thread_rng().gen_range(0.5, 1.0)),
                            thread_rng().gen_range(0.0, 0.5),
                        )))));
                } else {
                    hitable_list.push(Rc::new(Sphere::new(
                        center, 0.2, Rc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    hitable_list.push(Rc::new(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, Rc::new(
        Dielectric::new(1.5)
    ))));
    hitable_list.push(Rc::new(Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, Rc::new(
        Lambertian::new(Vec3(0.4, 0.2, 0.1))
    ))));
    hitable_list.push(Rc::new(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, Rc::new(
        Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)
    ))));


    HitableList::new(hitable_list)
}