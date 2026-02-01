#![allow(unused)]

// Iterators
fn main() {
    let vals: Vec<u32> = vec![1,2,3,4,5];
    // into_iter - iterate T
    // iter - iterate &T
    // iter_mut - iterate &mut T

    for v: u32 in vals.into_iter() {
        // 
    }
    for v: &u32 in vals.iter() {
        // 
    }
    for v: &mut u32 in vals.iter_mut() {
        // 
    }
}