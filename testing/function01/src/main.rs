#![allow(non_snake_case)]

fn changeString(s: &mut String) {
    *s = String::from("Bye, World!!!");
}

fn main() {
    let mut s = String::from("Hello, World!!!");
    println!("{}", s);
    changeString(&mut s);
    println!("{}", s);
}
