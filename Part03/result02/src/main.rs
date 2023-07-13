#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::fs::File;

fn main() {
    let f: u32 = File::open("hello.txt");
}
