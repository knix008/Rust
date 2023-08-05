#![allow(dead_code)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("Hello, world!");
}
