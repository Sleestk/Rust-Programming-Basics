#![allow(unused)]

// Trait
struct Foundry {
    version: String,
}

struct Cargo {
    version: String,
}

struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
    fn help(&self) -> String{
        "Good luck!".to_string()
    }
}

trait Tester {
    fn test(&self, file_path: &str) -> String;
}

fn test(tester: impl Tester, file_path: &str) -> String {
    tester.test(file_path)
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {}", file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {}", file_path)
    }
}

impl Compiler for Foundry {
    fn compile(&self, file_path: &str) -> String {
        format!("forge test {}", file_path)
    }
}

impl Compiler for Cargo {
    fn compile(&self, file_path: &str) -> String {
        format!("cargo test {}", file_path)
    }
}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn main() {
    let sol = Solidity {
        version: "0.8".to_string()
    };
    let vy = Vyper {
        version: "0.4".to_string()
    };
    let foundry = Foundry {
        version: "1.5.0".to_string()
    };
    let cargo = Cargo {
        version: "1.93.0".to_string()
    };

    println!("sol help: {}", sol.help());
    println!("vyper help: {}", vy.help());
    println!("foundry help: {}", foundry.help());
    println!("cargo help: {}", cargo.help());

    println!("sol compile: {}", sol.compile("hello.sol"));
    println!("vyper compile: {}", vy.compile("hello.vy"));
    println!("foundry compile: {}", foundry.compile("hello.foundry"));
    println!("cargo compile: {}", cargo.compile("hello.cargo"));


    // println!("sol compile: {}", compile(&sol, "hello.sol"));
    // println!("vyper compile: {}", compile(&vy, "hello.vy"));
}
// #[derive(Debug)]
// struct Point {
//     x: f32,
//     y: f32
// }

// impl Point {
//     // Static method - associated fucntion
//     fn new(x: f32, y: f32) -> Self {
//         Self {
//             x,
//             y
//         }
//     }

//     // Method
//     fn move_to(&mut self, x: f32, y: f32) {
//         self.x = x;
//         self.y = y;
//     }
// }

// fn main() {
//     // let mut p = Point {x: 0.0, y: 0.0};
//     let mut p = Point::new(0.0, 0.0);
//     p.move_to(1.0, 2.0);
//     println!("{:?}", p);
// }