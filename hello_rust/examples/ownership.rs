#![allow(unused)]

// Ownership rules
// 1. Each value has an owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

fn take(s: String) {
    println!("take {s}");
    // s is droppped
}

fn copy(v: i32) {
    println!("copy {v}");
    // v is dropped here
}

fn main() {
    // 1.
    // Owner of s is s
    let s = String::from("rust");
    // Owner of i is i
    let i = 1;

    // 2.
    let s = String::from("dog");
    // Owner of s is s1
    let s1: String = s;
    // Owner of s is s2
    let s2 = s1;
    println!("{s2}");
    // This will not compile
    // println!("{s1}");
    
    // 3.
    let s = String::from("cat");
    // introduces a new scope
    {
        s;
        // s is dropped at the end of the scope
    } // scope ends here
    // The code will not compile
    // println!("{s}");

    let s = String::from("cat");
    // introduces a new scope
    {
        // Owner of "cat" is s1
        let s1 = s;
        // s1 is dropped at the end of the scope
    } // scope ends here
    // The code will not compile
    // println!("{s}");

    let s = String::from("cat");
    take(s);

    // Ownership doesn't move for types that implement the Copy Trait
    // Owner of i is i
    let i = 1;
    // Owner of i1 is i1
    let i1 = i;
    // Owner of i2 is i2
    let i2 = i1;

    copy(i);
    println!("{i}");
}