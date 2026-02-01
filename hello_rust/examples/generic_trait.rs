#![allow(unused)]

trait List<T>{
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

trait Iterator<T> {
    fn next(&mut self) -> Option<&T>;
}

struct TupleIter<T> {
    t: (T, T, T),
    index: usize
}

struct VecIter<T> {
    v: Vec<T>,
    index: usize
}

impl<T> Iterator<T> for TupleIter<T> {
    fn next(&mut self) -> Option<&T> {
        let res = match self.index {
            0 => Some(&self.t.0),
            1 => Some(&self.t.1),
            2 => Some(&self.t.2),
            _ => None
        };
        self.index += 1;
        res
    }
}

impl<T> Iterator<T> for VecIter<T> {
    fn next(&mut self) -> Option<&T> {
        let res = match self.index {
            0 => Some(&self.v[0]),
            1 => Some(&self.v[1]),
            2 => Some(&self.v[2]),
            _ => None
        };
        self.index += 1;
        res
    }
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