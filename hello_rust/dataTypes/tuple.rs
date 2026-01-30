#![allow(unused)]

fn return_many() -> (u32, bool) {
    (1u32, true)
}

fn no_return() {}

fn return_empty_tuple() -> () {}

// Tuples = fixed size, mixed types, know at compile time
fn main() {
    let t: (bool, char, u32) = (true, 'a', 1);
    println!("{}, {}, {}", t.0, t.1, t.2);

    // Empty tuple = unit type
    // Result<(), String> = Ok(()) | Err("☠️")
    let t = ();

    // Nested tuple
    let nested = (('a', 1.23), (true, 1u32, -1i32), ());
    println!("nested.0.1: {}", (nested.0).1);

    // Destructuring a tuple
    let t: (bool, char, u32) = (true, 'a', 1);
    let (a, b, c) = t;
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Partial destructuring (ignore first and last values)
    let (_, b, _) = t;

    // Function that return multiple vlaues using a tuple
    let (a, b) =return_many();
}