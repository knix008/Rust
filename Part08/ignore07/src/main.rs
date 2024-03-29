#![allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 1, y: 2, z: 3 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
