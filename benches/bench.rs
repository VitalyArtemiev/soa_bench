#[macro_use]
extern crate bencher;

use bencher::Bencher;

use rand::Rng;

use soa_bench::types::Aos;

fn aos_bench(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut aos_vec = vec![Aos::default(); 10000];

    let mut dx: i32 = rng.gen::<i32>();
    let mut dy: i32 = rng.gen::<i32>();
    let mut dz: i32 = rng.gen::<i32>();

    bench.iter(||{
        for mut s in aos_vec.iter_mut() {
            dx = (dz + 57) % 512 - 256;
            dy = (dx + 57) % 512 - 256;
            dz = (dy + 57) % 512 - 256;

            s.x += dy;
            s.y += dy;
            s.z += dz;
        }
    })
}

benchmark_group!(benches, aos_bench);
benchmark_main!(benches);