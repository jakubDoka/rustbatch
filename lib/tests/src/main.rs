extern crate procedures;
use procedures::repeat;
use std::cell::RefCell;
use std::sync::Mutex;
use std::iter::Iterator;
use std::time::Instant;

#[derive(Debug)]
struct Foo {i: i32}

impl Foo {
    pub fn bar(mut self) {}
}

fn mv(a: &Vec<i128>) {}

fn main() {

    let t = Instant::now();
    let mut a = Vec::new();
    a.resize(100000000, 0i128);
    mv(&a);
    println!("{}",Instant::elapsed(&t).as_secs_f32())


}
