#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let result = read_username_from_file();
    println!("The result : {:?}", result);
}
