#![allow(non_snake_case)]
#![allow(unused_variables)]

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &mut v;
        
    first.push(6);
    println!("The v element is: {:?}", v);
}
