#[macro_use]
extern crate bencher;

use bencher::Bencher;

use rand::Rng;

use soa_bench::types::Aos;
use soa_bench::types::Soa;
use soa_bench::types::SoaVec;

const NUM_OBJECTS: usize = 1000000;

fn aos_bench(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut aos_vec = vec![Aos::default(); NUM_OBJECTS];

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

fn soa_bench(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut soa_vec = SoaVec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        soa_vec.push(Soa::default())
    }

    let mut dx: i32 = rng.gen::<i32>();
    let mut dy: i32 = rng.gen::<i32>();
    let mut dz: i32 = rng.gen::<i32>();

    bench.iter(||{
        for mut s in soa_vec.iter_mut() {
            dx = (dz + 57) % 512 - 256;
            dy = (dx + 57) % 512 - 256;
            dz = (dy + 57) % 512 - 256;

            *s.x += dy;
            *s.y += dy;
            *s.z += dz;
        }

        /*for i in 0..soa_vec.len() {
            dx = (dz + 57) % 512 - 256;
            dy = (dx + 57) % 512 - 256;
            dz = (dy + 57) % 512 - 256;

            soa_vec.x[i] += dx;
            soa_vec.y[i] += dy;
            soa_vec.z[i] += dz;
        }*/
    })
}

benchmark_group!(benches, aos_bench, soa_bench);
benchmark_main!(benches);