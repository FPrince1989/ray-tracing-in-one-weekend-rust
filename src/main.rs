use rand::thread_rng;
use ray_tracing::{draw, random_scene};
use std::fs;
use std::time::Instant;

fn main() {
    let mut rng = thread_rng();

    let nx = 1200;
    let ny = 800;
    let ns = 10;

    let world = random_scene(&mut rng);

    let start = Instant::now();
    println!("Start");
    let image_content = draw(&mut rng, nx, ny, ns, &world);

    println!("Cost: {:?}", start.elapsed());
    fs::write("ray_tracing.ppm", image_content).unwrap();
}
