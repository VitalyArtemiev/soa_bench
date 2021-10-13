use soa_bench::types::Test1;

mod types;

const NUM_OBJECTS: usize = 10000;

use soa_bench::types::{Test1Vec};


fn asm1() {
    let mut rng = rand::thread_rng();

    let mut soa_vec = Test1Vec::with_capacity(NUM_OBJECTS);
    for i in 0..NUM_OBJECTS {
        soa_vec.push(Test1::default())
    }

    for mut s in soa_vec.iter_mut() {
        *s.dx += 1;
        *s.dy += -1;
    }

    for _ in 1..10 {
        for mut s in soa_vec.iter_mut() {
            *s.dx += 1;
            *s.dy += -1;

            *s.x += *s.dx;
            *s.y += *s.dy;
        }
    }

    println!("{}", soa_vec.x[100])
}

fn asm2() {
    let mut rng = rand::thread_rng();

    let mut soa_vec = Vec::with_capacity(NUM_OBJECTS);
    for _ in 0..NUM_OBJECTS {
        soa_vec.push(Test1::default())
    }

    for mut s in soa_vec.iter_mut() {
        s.dx += 1;
        s.dy += -1;
    }

    for _ in 1..10 {
        for mut s in soa_vec.iter_mut() {
            s.dx += 1;
            s.dy += -1;

            s.x += s.dx;
            s.y += s.dy;
        }
    }

    println!("{}", soa_vec[100].x)
}

/*fn fuck1() {
    asm2();
    asm1();
    print!("f1");
    asm2();
}

pub fn fuck2() {
    asm1();
    print!("f2");
    asm2();
    asm1();
}*/

fn main() {
    for _ in 1..100 {
        asm1();
    }
    for _ in 1..100 {
        asm2();
    }
}
