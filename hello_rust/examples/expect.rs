#![allow(unused)]

// Unwrap and Expect
fn main() {
    // Unwrap
    let x: Option<i32> = Some(3);
    let v: i32 = match x {
        Some(val) => val,
        None => panic!("no value")
    };

    // Unwraps the inner value, Panics if None
    // let x: Option<i32> = None;
    let i = x.unwrap();
    println!("{i}");

    let x: Result<i32, String> = Ok(3);
    let v: i32 = match x {
        Ok(val) => val,
        Err(err) => panic!("err: {:?}", err)
    };

    // Cannot unwrap an error type
    // let x: Result<i32, String> = Err("error".to_string());
    let i = x.unwrap();
    println!("result: {}", i);

    // let x: Result<i32, String> = Err("something failed".to_string());
    // let v: i32 = match x {
    //     Ok(val) => val,
    //     Err(err) => panic!("this is the error message: {:?}", err)
    // };

    // Expect
    let x: Result<i32, String> = Err("something failed".to_string());
    x.expect("something failed");
    
}