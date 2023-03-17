use soa_derive::StructOfArray;
use derive_more::{Add, AddAssign, Sub, Display, From, Into};


#[derive(StructOfArray, Default)]
#[soa_derive(Debug, PartialEq)]
pub struct Soa {
    x: i32,
    y: i32,
    z: i32,
    name: String,
    flags: usize,
    color: (i32, i32, i32)
}
#[derive(Clone, Default)]
pub struct Aos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    name: String,
    flags: usize,
    color: (i32, i32, i32)
}

#[derive(StructOfArray, Default)]
#[soa_derive(Debug, PartialEq)]
pub struct Test1 {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    r: i32,
    g: i32,
    b: i32,
    name: String,
    flags: usize,
}

#[derive(StructOfArray, Default)]
#[soa_derive(Debug, PartialEq)]
pub struct Test2 {
    pos: (i32, i32),
    vel: (i32, i32),
    color: (i32, i32, i32),
    name: String,
    flags: usize
}

#[derive(StructOfArray, Default)]
#[soa_derive(Debug, PartialEq)]
pub struct Test3 {
    pos: [i32; 2],
    vel: [i32; 2],
    color: [i32; 3],
    name: String,
    flags: usize
}

#[derive(Debug, PartialEq, Default, Copy, Clone, Add, AddAssign)]
pub struct Vec2i {pub x: i32, pub y: i32}
#[derive(Debug, PartialEq, Default, Copy, Clone, Add, AddAssign)]
pub struct Vec3i {pub x: i32, pub y: i32, pub z: i32}

#[derive(StructOfArray, Default, Debug)]
#[soa_derive(Debug, PartialEq)]
pub struct Test4 {
    pos: Vec2i,
    vel: Vec2i,
    color: Vec3i,
    name: String,
    flags: usize
}