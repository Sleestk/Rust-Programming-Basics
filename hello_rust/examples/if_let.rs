#![allow(unused)]

// If Let - are you unwrapping an Option or Result?
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