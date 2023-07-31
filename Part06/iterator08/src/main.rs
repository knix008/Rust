#![allow(dead_code)]

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

fn main() {
    let x = Counter::new();

    println!("The counter : {:?}.", x);
}
