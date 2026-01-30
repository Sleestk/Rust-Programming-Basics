#![allow(unused)]

// Secular - data types that represent a single value
fn main() {
    // Signed integer
    // Range: -(2^(n - 1)) to 2^(n - 1) - 1
    // -2^(8-1) ~ 2n^(8-1) - 1
    let i0: i8 = -1;
    // -2^(16-1) ~ 2n^(16-1) - 1
    let i1: i16 = 2;
    // 2^(32-1) ~ 2n^(32-1) - 1
    let i2: i32 = 3;
    // 2^(64-1) ~ 2n^(64-1) - 1
    let i3: i64 = -4;
    // 2^(128-1) ~ 2n^(128-1) - 1
    let i4: i128 = 5;

    // Unsigned integer
    // Range: 0 to 2^(n) - 1
    // 0 ~ 2^16 - 1
    let u0: u8 = 1;
    // 0 ~ 2^16 - 1
    let u1: u16 = 2;
    // 0 ~ 2^32 - 1
    let u2: u32 = 3;
    // 0 ~ 2^64 - 1
    let u3: u64 = 4;
    // 0 ~ 2^128 - 1
    let u4: u128 = 5;

    // Depends on computer architecture
    let i5: isize = -6;
    let u5: usize = 6;

    // Floating point numbers
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // Boolean
    let b: bool = true;

    // Characters
    let c: char = 'c';
    let e: char = '🦀';

    // Type conversion 
    let i: i32 = -1;
    let u: u32 = i as u32;
    println!("{i} as u32 = {u}");

    // Min and max
    let i_max = i32::MAX;
    let u_min = u32::MIN;
    println!("i_max: {i_max}");
    println!("u_min: {u_min}");
}