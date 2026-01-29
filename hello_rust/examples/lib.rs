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

fn main() {
    println!("{}", eq('a', 'b')); // false
    println!("{}", eq('a', 'a')); // true

    let sum = add(1.0, 2.0, 3.0);
    println!("{}", sum);

    let result = cast(5, -2, 3.5);
    println!("{}", result);
}