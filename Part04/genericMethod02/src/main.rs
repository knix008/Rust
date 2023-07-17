#![allow(non_snake_case)]
#![allow(dead_code)]

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5.0, y: 10.0 };
    println!("The distance of origin : {}.", p.distance_from_origin());
}