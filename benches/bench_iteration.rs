#![feature(test)]
extern crate test;

use test::black_box;
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
        let mut c= 0;
        for mut s in soa_vec.iter_mut() {
            *s.color = Vec3i {x: c % 255, y: c % 255, z: c % 255};
            s.vel.x += 1;
            s.vel.y += -1;
            //*s.name = s.vel.y.to_string();

            *s.pos += *s.vel;
            c +=1;
            *s.flags += s.pos.y as usize;
        }
    });

    black_box(soa_vec);
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

    /*bench.iter(||{
        for i in 0..soa_vec.len() {
            soa_vec.vel[i].x += 1;
            soa_vec.vel[i].y += -1;

            soa_vec.pos[i] += soa_vec.vel[i];
        }
    });*/

    bench.iter(||{
        let mut c= 0;
        for i in 0..soa_vec.len() {
            soa_vec.color[i] = Vec3i {x: c % 255, y: c % 255, z: c % 255};
            soa_vec.vel[i].x += 1;
            soa_vec.vel[i].y += -1;
            //soa_vec.name[i] = soa_vec.vel[i].y.to_string();

            soa_vec.pos[i] += soa_vec.vel[i];
            c +=1;
            soa_vec.flags[i] += soa_vec.pos[i].y as usize;
        }
    });

    black_box(soa_vec);
}