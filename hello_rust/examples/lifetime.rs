#![allow(unused)] 

// Lifetime - every reference has a lifetime
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn print_refs<'a, 'b>(x: &'a str, y: &'b str) {
    println!("{} {}", x, y);
}

#[derive(Debug)]
// A string slice is a reference so we need a lifetime
struct Book<'a> {
    title: &'a str,   
}
// How about on a method?
impl<'a> Book<'a> {
    fn edit(&mut self, new_title: &'a str) {
        self.title = new_title;
    }
}

fn main() {
    let x = "Hello".to_string();
    let y = "Rust rust".to_string();
    let z = longest_str(&x, &y);
    println!("longest str: {}", z);

    // static lifetime
    let s: &'static str = "Hello";

    // Placeholder lifetime - let Rust infer the lifetime
    let s: &'_ str = "Rust";
}