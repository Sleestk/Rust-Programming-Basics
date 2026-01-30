// Cargo init ProjectName
// cargo build
// cargo fmt
// cargo test

fn main() {
    let s = String::from("Rust");
    let s1 = &s;
    let s2 = &s;
    println!("{s1}, {s2}");

    let mut s = String::from("Rust");
    let s1 = &mut s;
    // let s2 = &mut s;
    println!("{s1}");

    let s = String::from("Rust");
    println!("{s}");
}
