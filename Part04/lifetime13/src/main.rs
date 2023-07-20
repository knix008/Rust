fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x.clone()
    } else {
        y.clone()
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let mut result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}.", result);
    println!("The string1 address : {:p}.", &string1);
    println!("The string1 address : {:p}.", &string2);
    println!("The result  address : {:p}.", result);
    result = "abc";
    println!("The result  address : {:p}.", result);
    println!("The string of result: {}.", result);
}
