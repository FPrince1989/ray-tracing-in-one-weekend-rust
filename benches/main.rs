extern crate ray_tracing;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::thread_rng;
use ray_tracing::{draw, random_scene};

fn bench_draw(c: &mut Criterion) {
    let mut rng = thread_rng();

    let nx = 10;
    let ny = 10;
    let ns = 10;

    let world = random_scene(&mut rng);
    c.bench_function("draw", |b| b.iter(|| draw(&mut rng, nx, ny, ns, &world)));
}

criterion_group!(benches, bench_draw);

criterion_main!(benches);
