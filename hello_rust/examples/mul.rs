#![allow(unused)]

fn mul(x: u32, y: u32) -> u32 {
    x * y
}

fn main() {
    let x: u32 = 2;
    let y: u32 = 5;
    let z: u32 = mul(x, y);
    println!("{x} * {y} = {z}");
}