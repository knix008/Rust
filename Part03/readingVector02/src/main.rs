#![allow(non_snake_case)]
#![allow(unused_variables)]

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    match v.get(100) {
        Some(does_not_exist) => println!("The element is {}", does_not_exist),
        None => println!("There is no element."),
    }
}
