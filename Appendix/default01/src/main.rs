#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Default)]
#[derive(Debug)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

fn main() {
    let options: SomeOptions = Default::default();
    println!("The default option : {:?}.", options);
}
