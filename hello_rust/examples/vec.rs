#![allow(unused)]

// Vector
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    let v: Vec<i8> = vec![1, 2, 3];
    let v= vec![1u8, 2, 3];

    let v: Vec<i8> = vec![0i8; 100];
    println!("v: {:?}", v);

    // Get
    println!("v[1]: {}", v[1]);

    // Option<&i8>
    // Index valid => Some(&val)
    // Index invalid => None
    println!("v[1]: {:?}", v.get(1));
    println!("v[1]: {:?}", v.get(1000));

    // Update
    let mut v: Vec<i8> = vec![1, 2, 3];
    v[0] = 99;

    // pop - remove last element
    let mut v: Vec<i8> = vec![1, 2, 3];
    // 3
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x);
    // 2
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x);
    // 1
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x);
    // None
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x);

    // Slice
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..4];
    println!("slice: {:?}", s);

}