// // Overflow
// pub fn eq(a: char, b: char) -> bool {
//     a == b
// }
// pub fn add(c: f32, d: f32, e: f32) -> f32{
//     c + d + e
// }
// pub fn cast(x: u8, y: i8, z: f32) -> f32 {
//     let x = x as f32;
//     let y = y as f32;
//     x + y + z
// }

// // Tuples
// pub fn first(t: (bool, u32, char)) -> bool {
//     t.0
// }
// pub fn last(t: (bool, u32, char)) -> char {
//     t.2
// }
// pub fn swap(t: (u32, u32)) -> (u32, u32) {
//     (t.1, t.0)
// }

// // Arrays
// pub fn zeros() -> [u32; 100] {
//     [0; 100]
// }
// pub fn first_3(s: &[u32]) -> &[u32] {
//     &s[..3]
// }
// pub fn last_3(s: &[u32]) -> &[u32] {
//     &s[s.len() - 3 ..]
// }

// // Strings and &str
// pub fn hello() -> String {
//     String::from("Hello Rust")
// }
// pub fn greet(name: &str) -> String {
//     format!("Hello {}!", name)
// }
// pub fn append(mut s: String) -> String {
//     s += "!";
//     s
// }

// // Enum
// #[derive(Debug, PartialEq)]
// pub enum Color {
//     Red,
//     Green,
//     Blue,
//     Rgba(u8, u8, u8, f32)
// }

// // Struct
// // #[derive(Debug)]
// // pub struct Account {
// //     address: String,
// //     balance: u32
// // }
// // pub fn new(address: String) -> Account {
// //     Account {
// //         address,
// //         balance: 0,
// //     }
// // }

// // // Vec
// // pub fn init(x: u32, y: u32, z: u32) -> Vec<u32> {
// //     vec![x, y, z]
// // }

// // // HashMap
// // use std::collections::HashMap;
// // pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
// //     let mut map: HashMap<String, u32> = HashMap::new();
// //     map.insert(address, amount);
// //     map
// // }

// // If Else
// pub fn min(x: i32, y: i32) -> i32 {
//     if x < y { 
//         x
//     } else {
//         y
//     }
// }
// pub fn max(x: i32, y: i32) -> i32 {
//     if x < y { 
//         y
//     } else {
//         x
//     }
// }
// pub fn sign(x: i32) -> i32 {
//     if x < 0 { 
//         -1
//     } else {
//         1
//     }
// }

// // Loop
// pub fn sum(nums: Vec<i32>) -> i32 {
//     let sum: i32 = nums.iter().sum();
//     sum
// }
// pub fn fill(i: u32, n: usize) -> Vec<u32> {
//     vec![i; n]
// }

// // Match 
// pub fn num_to_string(num: u32) -> String {
//     match num {
//         1 | 2 | 3 => "one or two or three".to_string(),
//         _ => "ohter".to_string()
//     }
// }
// // pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
// //     match x {
// //         Some(n) => n,
// //         None => v
// //     }
// // }

// // If Let
// pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
//     if let Some(val) = x {
//         val
//     } else {
//         v
//     }
// }

// // Ownership
// pub fn exercise_1() {
//     let s = "rust".to_string();
//     let s1 = s;
//     // let s2 = s;
//     println!("{s1}");
// }
// pub fn exercise_2() {
//     let s = "rust".to_string();
//     {
//         let s1 = s;
//         println!("{s1}");
//     }
//     // println!("{s}");
// }
// pub fn exercise_3() {
//     let s = "rust".to_string();
//     // take(s);
//     println!("{s}");
//     println!("{s}");
// }

// // Error Handling
// #[derive(Debug)]
// pub enum MathError {
//     DivByZero
// }
// pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
//     if y == 0 {
//         Err(MathError::DivByZero)
//     } else {
//         Ok ( x / y )
//     }
// }
// // return v[i] if i is a valid index, otherwise return default_val
// pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
//     match v.get(i) {
//         Some(&val) => val,
//         None => default_val
//     }
// }

// // Unwrap and Expect
// pub fn parse_and_add(a: &str, b: &str) -> u32 {
//     let a_num: u32 = a.parse().expect("Failed to parse variable");
//     let b_num: u32 = b.parse().expect("Failed to parse variable");
//     a_num + b_num
// }
// pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
//     x.unwrap() + y.unwrap()
// }

// // Question operator - ?
// fn parse_u32(s: &str) -> Result<u32, String> {
//     Ok(s.parse::<u32>().map_err(|e| e.to_string())?)
// }
// pub fn sum(nums: &[&str]) -> Result<u32, String> {
//     let mut total = 0;
//     for n in nums {
//         let value = parse_u32(n)?;
//         total += value;
//     }
//     Ok(total)
// }

// Generic Types
// pub fn first<T, U>(t: (T, U)) -> T {
//     t.0
// }
// pub fn last<T, U>(t: (T, U)) -> U {
//     t.1
// }
// #[derive(Debug)]
// pub struct Rectangle<T> {
//     pub top: T,
//     pub left: T,
//     pub width: T,
//     pub height: T
// }

fn main() {

    // // Question operator - ?
    // let nums = ["10", "20", "30"];
    // println!("{:?}", sum(&nums));

    // let bad = ["10", "abc", "30"];
    // println!("{:?}", sum(&bad));

    // // Unwrap and Expect
    // let res = parse_and_add("10", "20");
    // println!("result: {}", res);
    // let res = unwrap_and_add(Some(50), Some(20));
    // println!("result: {}", res);

    // // Error Handling
    // let nums = vec![10, 20, 30];
    // let res = get(&nums, 2, 0);
    // println!("{:?}", res);

    // let x = 10;
    // let y = 0;
    // let res = div(x, y);
    // println!("{:?}", res);

    // // If Let
    // println!("{}", unwrap_or_default(Some(10), 99));
    // println!("{}", unwrap_or_default(None, 99));

    // // Match
    // println!("{}", num_to_string(1));
    // println!("{}", num_to_string(25));
    // println!("{}", unwrap_or_default(Some(10), 99));
    // println!("{}", unwrap_or_default(None, 99));

    // // Loop
    // let nums = vec![ 3, 12, 5, 6, 9, 10 ];
    // println!("Sum of integers: {}", sum(nums));
    // println!("Vec len: {:?}", fill(0, 10));
    // println!("Vec len: {}", fill(0, 10).len());

    // // If Else
    // println!("{}", min(10, 20));
    // println!("{}", max(10, 20));
    // println!("{}", sign(0));

    // // Hash Map
    // let balances = init("0xABC".to_string(), 100);
    // println!("{:?}", balances);

    // // Vec
    // println!("{:?}", init(1, 2, 3));

    // // Struct
    // let a = Account { 
    //     address: "123 Rust Lane".to_string(),
    //     balance: 100
    // };
    // println!("{:?}, {:?}", a.address, a.balance);

    // // Enum
    // // Assign simple "unit" variants
    // let sky_color = Color::Blue;
    // let stop_color = Color::Red;
    // // Assign the "tuple" variant with data
    // let sunset_orange = Color::Rgba(255, 165, 0, 1.0);
    // // Print them using Debug trait
    // println!("{:?}", sky_color);
    // println!("{:?}", sunset_orange);
    // println!("{:?}", stop_color);
    // // Using the PartialEq trait to compare values
    // if stop_color == Color::Red {
    //     println!("Stop light is red! STOP!"); 
    // }

    // // Strings and &str
    // println!("{}", hello());
    // println!("{}", greet("Rust"));
    // println!("{}", append(String::from("Hello Rust")));

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