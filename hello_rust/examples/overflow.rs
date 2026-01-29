#![allow(unused)]

// Overflwo doesn't panic when compiled with --release
fn main() {
    let mut x = u32::MAX;
    x += 1;

    println!("u32 max: {}, x: {}", u32::MAX, x);

    // u32::checked_add - return None on overflow
    let x = u32::checked_add(3, 1);
    println!("checked_add: {:?}", x);
    // u32::wrapping_add - explicitly allow overflow
    let x = u32::wrapping_add(3, 1);
    println!("wrappping_add: {:?}", x);
}