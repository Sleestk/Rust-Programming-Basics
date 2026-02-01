#![allow(unused)]

trait List<T>{
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for (u32, bool, char) {
    fn count(&self) -> usize {
        3
    }

    fn first(&self) -> &u32 {
        &self.0
    }
}

impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }

    fn first(&self) -> &T {
        &self[0]
    }
}

fn main() {
    let t = (1u32, true, 't');
    println!("count: {}", t.count());
    println!("first: {}", t.first());

    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    println!("count: {}", v.count());
    println!("first: {}", v.first());
}