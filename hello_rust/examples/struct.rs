#![allow(unused)]

// Struct
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

struct Point3D(i32, i32, i32);

struct Empty;

#[derive(Debug)]
struct Circle {
    radius: u32,
    center: Point
}

fn main() {
    let p= Point { x: 1, y: 1};
    println!("{:?}", p);
    println!("x: {}, y: {}", p.x, p.y);

    let p = Point3D(-1, 0, 1);
    println!("point 3D: ({}, {}, {})", p.0, p.1, p.2);

    let empty = Empty;

    let circle = Circle{
        radius: 1,
        center: Point{x: 0, y: 0}
    };
    println!("{:#?}", circle);

    // Shortcut
    let x: i32 = 1;
    let y: i32 = 1;
    let p = Point { x: x, y: y};
    let p = Point { x, y };

    // Copy fields
    let p0 = Point { x: 1, y: 2};
    let p1 = Point { x: 1, y: p0.y};
    let p1 = Point {x: 2, ..p0};
    println!("p1 copy: {:?}", p1);

    // Update
    let mut p= Point { x: 1, y: 1}; 
    p.x += 1;
    p.y = 99;
    println!("update point: {:?}", p);

}