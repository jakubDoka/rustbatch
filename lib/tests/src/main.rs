extern crate procedures;
use procedures::repeat;
use std::cell::RefCell;
#[derive(Debug)]
struct Foo {i: i32}

impl Foo {
    pub fn bar(mut self) {}
}

fn br(f: Foo) {}

fn main() {
    let bts = 100u16.to_be_bytes();
    let mut a = 0u16;
    for b in bts.iter() {
        a = a ^ (*b as u16)
    }

    println!("{}", a)
}
