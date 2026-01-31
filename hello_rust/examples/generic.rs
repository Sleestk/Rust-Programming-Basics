#![allow(unused)]

// Option, Result, Vec
// enum Option<T> {
//     Some(T),
//     None
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

struct Point<T> {
    x: T,
    y: T
}

fn swap<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

fn main() {
    let v: Vec<i32> = vec![1i32, 2, 3];

    let p : Point<f32> = Point {x: 0.0, y: 0.0};

    let a: u32 = 1;
    let b: i32 = 2;
    let (a, b) = swap(a, b);
}