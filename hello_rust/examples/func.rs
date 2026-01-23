#![allow(unused)]

fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y;
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn print(s: String) {
    println!("{s}{s}{s}{s}{s}");
}

// Exercise 1
pub fn mul() {
    let x: u32 = 2;
    let y: u32 = 5;
    let z: u32 = x * y;
    println!("{x} * {y} = {z}");
}

// Exercise 1
pub fn div() {
    let x: u32 = 2;
    let y: u32 = 5;
    let z: u32 = x / y;
    println!("{x} / {y} = {z}");
}

fn main() {
    let x: u32 = 1;
    let y: u32 = 2;
    let z: u32 = add(x, y);
    println!("{x} + {y} = {z}");

    print("😋".to_string());
}