#[macro_use]
extern crate bencher;

use bencher::Bencher;

use rand::Rng;

use soa_bench::types::{Aos, Test1, Test1Vec, Test2, Test2Vec, Test3, Test3Vec, Test4, Test4Vec};
use soa_bench::types::Soa;
use soa_bench::types::SoaVec;

const NUM_OBJECTS: usize = 1000000;

fn bench0(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut soa_vec = Vec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        soa_vec.push(Test1::default())
    }

    for mut s in soa_vec.iter_mut() {
        s.dx += 1;
        s.dy += -1;
    }

    bench.iter(||{
        for mut s in soa_vec.iter_mut() {
            s.dx += 1;
            s.dy += -1;

            s.x += s.dx;
            s.y += s.dy;
        }
    })
}

fn bench1_sep(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut soa_vec = Test1Vec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        soa_vec.push(Test1::default())
    }

    for mut s in soa_vec.iter_mut() {
        *s.dx += 1;
        *s.dy += -1;
    }

    bench.iter(||{
        for mut s in soa_vec.iter_mut() {
            *s.dx += 1;
            *s.dy += -1;

            *s.x += *s.dx;
            *s.y += *s.dy;
        }
    })
}

fn bench2_tup(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut soa_vec = Test2Vec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        soa_vec.push(Test2::default())
    }

    for mut s in soa_vec.iter_mut() {
        s.vel.0 += 1;
        s.vel.1 += -1;
    }

    bench.iter(||{
        for mut s in soa_vec.iter_mut() {
            s.vel.0 += 1;
            s.vel.1 += -1;

            s.pos.0 += s.vel.0;
            s.pos.1 += s.vel.1;
        }
    })
}

fn bench3_arr(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut soa_vec = Test3Vec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        soa_vec.push(Test3::default())
    }

    for mut s in soa_vec.iter_mut() {
        s.vel[0] += 1;
        s.vel[1] += -1;
    }

    bench.iter(||{
        for mut s in soa_vec.iter_mut() {
            s.vel[0] += 1;
            s.vel[1] += -1;

            s.pos[0] += s.vel[0];
            s.pos[1] += s.vel[1];
        }
    })
}

fn bench4_str(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let mut soa_vec = Test4Vec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        soa_vec.push(Test4::default())
    }

    for mut s in soa_vec.iter_mut() {
        s.vel.x += 1;
        s.vel.y += -1;
    }

    bench.iter(||{
        for mut s in soa_vec.iter_mut() {
            s.vel.x += 1;
            s.vel.y += -1;

            s.pos.x += s.vel.x;
            s.pos.y += s.vel.y;
        }
    })
}



benchmark_group!(benches, bench3_arr, bench1_sep, bench2_tup, bench0, bench4_str);
benchmark_main!(benches);