#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other
}

// Error handling
// panic!
// Option<T> = Some(T) | None
// Result<T, E> = Ok(T) | Err(E)

// Error
fn main () {
    // panic!("something went wrong");

    let v = vec![1, 2, 3];
    // Index out of bounds
    // v[99];

    let x: Option<&i32> = v.get(1);
    match x {
        Some(val) => println!("x: {:?}", val),
        None => println!("x: None")
    }

    // Result<T, E> = Ok(T) | Error(E)
    let x = 10;
    let y = 0;
    // This will panic. Division buy 0
    // let q = x / y;

    

    let q: Result<i32, MathError> = if y != 0 {
        Ok(x / y)
    } else {
        Err(MathError::DivByZero)
    };

    match q {
        Ok(val) => println!(" x / y = {:?}", val),
        Err(err) => println!("x / y error {:?}", err)
    }
}