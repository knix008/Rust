#![allow(non_snake_case)]

fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..1];

    println!("The slice : {}", s);
}