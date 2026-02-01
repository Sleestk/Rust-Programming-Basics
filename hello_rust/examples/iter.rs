#![allow(unused)]

use::std::collections::HashMap;

fn filter_non_zero(v: Vec<i32>) -> Vec<i32> {
    v.iter().filter(|x: &&i32| **x > 0).map(|x: &i32| x - 3).collect()
}

// Iterators
// - map, filter, collect
fn main() {
    let vals: Vec<i32> = vec![1,2,3,4,5];
    println!("filter non zero: {:?}", filter_non_zero(vals));
    // //&u32
    // // map - f(x: &u32) -> u32
    // let v2: Vec<u32> = vals.iter().map(|x| x + 1).collect();
    // println!("v2: {:?}", v2);

    //collect
    let vals: Vec<(&str, u32)> = vec![("a", 1), ("b", 2), ("c", 3)];
    let v: Vec<(String, u32)> = vals.iter().map(|v| (v.0.to_string(), v. 1 + 1)).collect();
    println!("vec: {:?}", v);

    let v: HashMap<String, u32> = vals.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    println!("hashmap: {:?}", v);

    // Chaining filter and map
    let vals: Vec<u32> = vec![1,2,3,4,5];
    // &T
    let v: Vec<u32> = vals.iter().filter(|x: &&u32| **x <= 3).map(|x: &u32| x + 1).collect();
    println!("filter -> map: {:?}", v);
    // into_iter - iterate T example
    // let v: Vec<u32> = vals.into_iter().filter(|x: &u32| *x <= 3).map(|x: u32| x + 1).collect();
    // println!("filter -> map: {:?}", v);


    // let vals: Vec<u32> = vec![1,2,3,4,5];
    // // into_iter - iterate T
    // // iter - iterate &T
    // // iter_mut - iterate &mut T

    // for v: u32 in vals.into_iter() {
    //     // 
    // }
    // for v: &u32 in vals.iter() {
    //     // 
    // }
    // for v: &mut u32 in vals.iter_mut() {
    //     // 
    // }
}