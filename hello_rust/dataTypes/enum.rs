#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize{width: u32, height: u32}
}

// Enum
fn main() {
    let cmd: Command = Command::Play;
    let cmd: Command = Command::Skip(10);
    let cmd: Command = Command::Resize { width: (100), height: (50) };
    // Debug
    println!("{:?}", cmd);

    // PartialEqs
    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);
    println!("{:?}", cmd0 == cmd0);

    // Option<T> = Some(T) | None
    let x: Option<i32> = Some(1);
    let x: Option<i32> = None;

    // Result<T, E> = Ok(T) | Error(E)
    // "100" -> 100
    let x: Result<i32, String> = Ok(100);
    // "123afojn?" -> error
    let x: Result<i32, String> = Err(
        "Failed to parse string into number".to_string()
    );
}