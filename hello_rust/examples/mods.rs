#![allow(unused)]

use hello_rust::{foo, my};

fn main() {
    my::print();
    my::a::print();

    let s = my::a::build(1);

    my::a::print_foo();
}