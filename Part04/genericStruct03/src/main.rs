#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("The integer point : {:?}.", both_integer);
    println!("The float point : {:?}.", both_float);
    println!("The integer and float point : {:?}.", integer_and_float);
}
