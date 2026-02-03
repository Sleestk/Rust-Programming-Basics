#!{allow(unused)}

// Match - are you branching on an enum cariant or pattern?
fn main() {
    let x: i32 = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }

    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other")
    }

    let x = 10;
    match x {
        i @ 1..=10 => println!("1 to 10 {i}"),
        _ => println!("other")
    }

    let x: Option<i32> = Some(9);
    match x {
        Some(val) => println!("Option is {val}"),
        None => println!("None")
    }

    let res: Result<i32, String> = Ok(100);
    // let res: Result<i32, String> = Err("failed".to_string());
    match res {
        Ok(val) => println!("ok {val}"),
        Err(err) => println!("error {err}")
    }

    let x: Option<i32> = Some(9);
    // let x: Option<i32> = None;
    let z: i32 = match x{
        Some(val) => val,
        None => 0
    };
    println!("match returned {}", z);
}