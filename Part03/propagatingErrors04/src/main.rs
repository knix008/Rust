#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let result = read_username_from_file();
    println!("The result : {:?}", result);
}
