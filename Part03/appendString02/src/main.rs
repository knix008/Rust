#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

fn main() {
    let mut s = String::from("lo");

    s.push('l');
    println!("1. The string : {}", s);
    //s.push('LL');   /* This line will make a compile error. */
    //println!("2. The string : {}", s);
}