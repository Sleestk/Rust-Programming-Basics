#![allow(unused)]

// Memory - stack and heap
// Stack
// - Stores data of fixed size known at compile time
// - Fast
// - LIFO (list in, first out)
//  C
// ---
//  B
// ---
//  A

// Heap
// - Stores data of unkown size at compile time
// - Slower than stack
// - Memory safety is enforced through Rust's ownership and borrowing rules
fn main() {
    // Stack
    let x: i32 = 1;
    let arr: [i32; 10] =[1; 10];

    // Heap
    let mut s: String = "hello rust".to_string();
    s += "!";

    let mut v = vec![];
    v.push(1);
    v.push(2);
    v.push(3);

    // force any data type to be stored on the heap
    let boxed = Box::new(1i32);

}