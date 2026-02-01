#![allow(unused)]
use std::cmp::PartialOrd;

// Trait Bound

fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x <=y {
        x
    } else {
        y
    }
}

fn zip<T: Copy, U: Copy>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> {
    let mut v = vec![];
    let len = min(a.len(), b.len());

    for i in 0..len {
        v.push((a[i], b[i]));
    }
    v
}

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for u32 {}

fn a<T: A>(x: T) {}

fn ab<T: A + B>(x: T) {}

// fn w<T: A + B, U: B + C>(x: T, y: U) {}
fn w<T, U>(x: T, y: U) where
    T: A + B,
    U: B + C
{}

fn min<T: PartialOrd> (x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

fn main() {
    let u: u32 = 1;
    let i: i32 = -1;
    let f: f32 = 1.0;
    a(u);
    // a(i);

    ab(u);
    // ab(f);
}