#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let option : Option<i32> = Option::Some(1);

    println!("The integer Option<T> : {:?}.", option);
}
