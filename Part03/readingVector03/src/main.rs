#![allow(non_snake_case)]
#![allow(unused_variables)]

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let y = v.get(100);

    match v.get(100) {
        Some(y) => println!("The element is {}", y),
        None => println!("There is no element."),
    }
}
