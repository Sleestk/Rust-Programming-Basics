#![allow(unused)]

use std::collections::HashMap;

// HashMap
fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("red".to_string(), 100);
    scores.insert("blue".to_string(), 200);
    println!("{:#?}", scores);

    // Get
    let score: Option<&u32> = scores.get("red");
    println!("Red score: {:?}", score);
    let score: Option<&u32> = scores.get("green");
    println!("Green score: {:?}", score);

    //Update
    let score: &mut u32 = scores.entry("blue".to_string()).or_insert(0); 
    *score += 100;

    let score: Option<&u32> = scores.get("blue");
    println!("Blue score: {:?}", score);

}