#![allow(unused)]

#[derive(Debug)]
// Method
struct Point {
    top: u32,
    left: u32,
}

impl Point {
    // // Static method - associated fucntion
    // fn new(top: u32, left: u32) -> Self {
    //     Self {
    //         top,
    //         left
    //     }
    // }
    // Method
    fn move_to(&mut self, top: u32, left: u32) {
        self.top = top;
        self.left = left;
    }
}
fn main() {
    let mut p: Point = Point {top: 1, left: 2};
    p.move_to(10, 20);
    println!("{:?}", p);
}