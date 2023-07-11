#![allow(non_snake_case)]

fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("The slice : {}", s);
}