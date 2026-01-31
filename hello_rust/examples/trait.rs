#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

impl Point {
    // Static method - associated fucntion
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }

    // Method
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    // let mut p = Point {x: 0.0, y: 0.0};
    let mut p = Point::new(0.0, 0.0);
    p.move_to(1.0, 2.0);
    println!("{:?}", p);
}