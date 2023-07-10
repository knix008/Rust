#![allow(non_snake_case)]

fn main() {
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    println!("The vector : {:?}.", v);
}
