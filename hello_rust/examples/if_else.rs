#![allow(unused)]

// If / else - are you checking a single condition and going one way or the other?
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