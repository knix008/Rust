#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };

    println!("The integer point : {:?}.", wont_work);
}
