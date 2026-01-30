#![allow(unused)]

// Loop
fn main() {
    let mut i = 0;
    loop {
        println!("loop {i}");
        i += 1;
        if i > 5 {
            break;
        }
    }

    let mut i = 0;
    while i <= 5 {
        println!("while loop {i}");
        i += 1;
    }

    for i in 0..6 {
        println!("for loop {i}");
    }

    let arr = [1, 2, 3, 4, 5];

    let n: usize = arr.len();
    for i in 0..n {
        println!("arr {}", arr[i]);
    }

    for n in arr {
        println!("arr {}", n);
    }

    let v = vec![10, 20, 30, 40, 50];
    for n in v.iter() {
        println!("vec {}", n);
    }

    for n in v.into_iter() {
        println!("vec {}", n);
    }

    let mut i = 0;
    let z: &str =loop {
        println!("loop {i}");
        i += 1;
        if i > 5 {
            break "loop ends here";
        }
    };
    println!("loop returns {z}");

}