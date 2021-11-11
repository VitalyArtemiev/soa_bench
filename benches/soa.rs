#![feature(test)]
extern crate test;

use soa_derive::StructOfArray;
use test::Bencher;

#[derive(StructOfArray)]
pub struct Small {
    x: f64,
    y: f64,
    z: f64,
}

impl Small {
    fn new() -> Small {
        Small {
            x: 1.0,
            y: 0.2,
            z: -2.3,
        }
    }

    fn aos_vec(size: usize) -> Vec<Small> {
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(Small::new())
        }
        return vec;
    }

    fn soa_vec(size: usize) -> SmallVec {
        let mut vec = SmallVec::new();
        for _ in 0..size {
            vec.push(Small::new())
        }
        return vec;
    }
}

#[derive(StructOfArray)]
pub struct Big {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
    id: usize,
    name: String,
    userdata: String
}

impl Big {
    fn new() -> Big {
        Big {
            position: (1.0, 0.2, -2.3),
            velocity: (1.0, 0.2, -2.3),
            id: 67,
            name: "foo".into(),
            userdata: "bar".into()
        }
    }

    fn aos_vec(size: usize) -> Vec<Big> {
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(Big::new())
        }
        return vec;
    }

    fn soa_vec(size: usize) -> BigVec {
        let mut vec = BigVec::new();
        for _ in 0..size {
            vec.push(Big::new())
        }
        return vec;
    }
}

#[bench]
fn aos_small_push(bencher: &mut Bencher) {
    let mut vec = Vec::new();
    bencher.iter(||{
        vec.push(Small::new())
    })
}

#[bench]
fn soa_small_push(bencher: &mut Bencher) {
    println!("9");

    let mut vec = SmallVec::new();
    bencher.iter(||{
        vec.push(Small::new())
    })
}

#[bench]
fn aos_big_push(bencher: &mut Bencher) {
    println!("8");

    let mut vec = Vec::new();
    bencher.iter(||{
        vec.push(Big::new())
    })
}

#[bench]
fn soa_big_push(bencher: &mut Bencher) {
    println!("7");

    let mut vec = BigVec::new();
    bencher.iter(||{
        vec.push(Big::new())
    })
}

#[bench]
fn aos_small_do_work_10000(bencher: &mut Bencher) {
    println!("6");

    let vec = Small::aos_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for v in &vec {
            s += v.x + v.y;
        }
        s
    })
}

#[bench]
fn soa_small_do_work_10000(bencher: &mut Bencher) {
    println!("5");

    let vec = Small::soa_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for (x, y) in vec.x.iter().zip(&vec.y) {
            s += x + y;
        }
        s
    })
}

#[bench]
fn aos_big_do_work_1000(bencher: &mut Bencher) {
    println!("4");

    let vec = Big::aos_vec(1000);
    bencher.iter(||{
        let mut s = 0.0;
        for v in &vec {
            s += v.position.0 + v.velocity.0 * 0.1;
        }
        s
    })
}

#[bench]
fn aos_big_do_work_10000(bencher: &mut Bencher) {
    println!("3");

    let vec = Big::aos_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for v in &vec {
            s += v.position.0 + v.velocity.0 * 0.1;
        }
        s
    })
}

#[bench]
fn soa_big_do_work_1000(bencher: &mut Bencher) {
    println!("2");

    let vec = Big::soa_vec(1000);
    bencher.iter(||{
        let mut s = 0.0;
        for (position, velocity) in vec.position.iter().zip(&vec.velocity) {
            s += position.0 + velocity.0;
        }
        s
    })
}

#[bench]
fn soa_big_do_work_10000(bencher: &mut Bencher) {
    println!("1");

    let vec = Big::soa_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for (position, velocity) in vec.position.iter().zip(&vec.velocity) {
            s += position.0 + velocity.0;
        }
        s
    })
}

/*fn soa_big_write_10000(bencher: &mut Bencher) {
    let vec = Big::soa_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for (position, velocity) in vec.position.iter_mut().zip(&vec.velocity) {
            s += position.0 + velocity.0;
        }
        s
    })
}*/

