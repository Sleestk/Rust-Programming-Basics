#![allow(unused)]

// If Let
fn main () {
    let x: Option<i32> = Some(9);
    match x {
        Some(val) => println!("Option is {val}"),
        None => {}
    }

    if let Some(val) = x {
        println!("Option is {val}");
    }
}