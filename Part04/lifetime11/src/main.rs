#![allow(unused_variables)]

fn longest<'b>(x: &'b str, y: &str) -> &'b str {
    x
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}
