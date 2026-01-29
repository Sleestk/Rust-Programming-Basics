#![allow(unused)]

// Strings and &str (string slice)
// - Use 'String' when you need ownership or mutability
// - Use '&str' for read-only string or string literals
fn main() {
    let msg: String = String::from("Hello Rust");
    let msg: String = "Hello Rust".to_string();

    let lenght: usize = msg.len();

    let msg: String = String::from("Hello Rust");
    let s: &str = &msg[0..5];
    println!("s = {}", s);

    let s: &str = "Hello World";
    let x: String = s.to_string();

    // Rust automatically converts &String into a &str
    let msg: String = String::from("Hello Rust");
    print(&msg);

    let s: &str = "Hello World";
    print(s);

    // Append &str to String
    let mut msg: String = String::from("Hello Rust");
    msg += " World";
    println!("{msg}");

    // String interpoolation - format!
    let lang = "Rust";
    let emoji = "🦀";
    let s = "Hello Rust 🦀";
    let mut s = "Hello".to_string();
    s += " ";
    s += lang;
    s += " ";
    s += emoji;
    let s = format!("Hello {} {}", lang, emoji);
    println!("{s}");


}

fn print(s: &str) {
    println!("{s}");
}