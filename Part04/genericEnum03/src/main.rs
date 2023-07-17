#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let result : Result<i32, i32> = Result::Ok(0);
    println!("The result : {:?}.", result);
    let result : Result<i32, i32> = Result::Err(1);
    println!("The result : {:?}.", result);
}
