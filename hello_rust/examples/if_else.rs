#![allow(unused)]

// If / else
fn main() {
    let x: i32 = 10;

    let z: i32 = if x > 0 {
        println!("x > 0");
        1
    } else if x < 0 {
        println!("x < 0");
        -1
    } else {
        println!("x = 0");
        0
    };
    println!("z: {}", z);
}