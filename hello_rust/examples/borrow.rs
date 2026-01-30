#![allow(unused)]

// Borrowing
fn take(s: String) {
    println!("take {s}");
}

fn borrow(s: &String) {
    println!("borrow {s}");
}

// Borrow - temporarily use a value without tkaing ownership
// - Creates a referemce (either mutable or immutable)
// - Doesn't move ownership
// - Immutable reference - any number of read-only access to a value 
// - Mutable reference - only one read and write access toa value at a time
// - Either immutable or mutable borrow, but not both simultaneously
// Reference must not outlive the value

fn main() {
    // Take ownership
    let s = String::from("rust");
    borrow(&s);
    println!("{s}");

    // - Creates a referemce (either mutable or immutable)
    // Immutable borrow
    // - Immutable reference - any number of read-only access to a value 
    let s = String::from("rust");
    // s1, s2, and s3 have read-only access to s
    let s1 = &s;
    let s2 = &s;
    let s3 = s2;

    // Mutable borrow
    let mut s = String::from("rust");
    // - Mutable reference - only one read and write access to a value at a time
    // let s1 = &mut s;
    // let s2 = &mut s;
    // s1.push_str("🦀");
    // s2.push_str("🦀");
    // ^^^^^^^^^^^^^^^^^ Will not compile
    let s1 = &mut s;
    // s1 has read and write access to s
    s1.push_str("🦀");
    let s2 = &mut s;
    s2.push_str("🦀");
    // ^^^^^^^^^^^^^^^^^^^^^^ Will compile

    // - Either immutable or mutable borrow, but not both simultaneously
    // let mut s = String::from("rust");
    // s1, s2, and s3 have read-only access to s
    // let s1 = &s;
    // let s2 = &s;
    // let s3 = &mut s;
    // println!("s1: {s1}");
    // s3.push_str("🦀");
    
    // Reference must not outlive the value
    // let s = String::from("rust");
    // let s1 = &s;
    // {
    //     let s2 = s;
    // } // s2 will be dropped
    // // s2 and s no longer exist
    // // s1 reference s
    // println!("s1: {s1}");
}

// Reference must not outlive the value pt.2
// fn dangle(s: String) -> &String {
//     &s
//     // returning a reference, but the value is already dropped
// } 