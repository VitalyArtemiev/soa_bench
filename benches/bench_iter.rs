#![feature(test)]
extern crate test;

use soa_bench::types::*;
use test::Bencher;

const NUM_OBJECTS: usize = 1000000;

#[bench]
fn bench_iter(bench: &mut Bencher) {
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

            *s.pos += *s.vel;
        }
    })
}

#[bench]
fn bench_for(bench: &mut Bencher) {
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
        for i in 0..soa_vec.len() {
            soa_vec.vel[i].x += 1;
            soa_vec.vel[i].y += -1;

            soa_vec.pos[i] += soa_vec.vel[i];
        }
    })
}