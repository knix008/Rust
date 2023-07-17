#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("The integer point : {:?}.", integer);
    println!("The float point : {:?}.", float);
}
