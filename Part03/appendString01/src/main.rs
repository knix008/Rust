#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
}