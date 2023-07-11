#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("The string : {}", s);
}