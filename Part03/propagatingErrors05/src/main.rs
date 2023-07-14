#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;

    println!("The file handle : {:?}", f);
}
