// Overflow
pub fn eq(a: char, b: char) -> bool {
    a == b
}
pub fn add(c: f32, d: f32, e: f32) -> f32{
    c + d + e
}
pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    let x = x as f32;
    let y = y as f32;
    x + y + z
}

// Tuples
pub fn first(t: (bool, u32, char)) -> bool {
    t.0
}
pub fn last(t: (bool, u32, char)) -> char {
    t.2
}
pub fn swap(t: (u32, u32)) -> (u32, u32) {
    (t.1, t.0)
}

// Arrays
pub fn zeros() -> [u32; 100] {
    [0; 100]
}
pub fn first_3(s: &[u32]) -> &[u32] {
    &s[..3]
}
pub fn last_3(s: &[u32]) -> &[u32] {
    &s[s.len() - 3 ..]
}

// Strings and &str
pub fn hello() -> String {
    String::from("Hello Rust")
}
pub fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}
pub fn append(mut s: String) -> String {
    s += "!";
    s
}

fn main() {
    // Strings and &str
    println!("{}", hello());
    println!("{}", greet("Rust"));
    println!("{}", append(String::from("Hello Rust")));

    // // Arrays
    // println!("arr: {:?}", zeros());
    // let arr = [2, 5, 6, 3, 1];
    // println!("first_3: {:?}", first_3(&arr));
    // println!("last_3: {:?}", last_3(&arr));

    // // Tuples
    // println!("first element in tuple: {}", first((true, 42, 'a')));
    // println!("last element in tuple: {}", last((true, 42, 'a')));
    // println!("Swap elements: {:?}", swap((10, 2)));

    // // Overflow
    // println!("{}", eq('a', 'b')); // false
    // println!("{}", eq('a', 'a')); // true
    // let sum = add(1.0, 2.0, 3.0);
    // println!("{}", sum);
    // let result = cast(5, -2, 3.5);
    // println!("{}", result);
}