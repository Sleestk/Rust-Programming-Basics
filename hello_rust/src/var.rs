#![allow(unused)]

// Constacnts
const NUM: u32 = 1;

fn main() {
    // Variables
    // - Immutable by default
    // - Use mut keyword to make it mutable
    let mut x = 1;
    x += 1;

    let y: i32 = -1;
    let z = -1;

    // Shadowing
    let x: i32 = 1;
    let x: i32 = 2;
    let x: bool = true;

    // Type placeholders
    let x: _ = 1234;
   
    // println!
    let x = 1;
    println!("x is {}", x);
}