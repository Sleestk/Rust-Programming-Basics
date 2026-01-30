#![allow(unused)]

fn div(x: u32, y: u32) -> u32 {
    x / y
}

fn main() {
    let x: u32 = 10;
    let y: u32 = 2;
    let z: u32 = div(x, y);
    println!("{x} / {y} = {z}");
}