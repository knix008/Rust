#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::str;
fn main() {
    // some bytes, in a vector
    let s = vec![224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135];

    // We know these bytes are valid, so just use `unwrap()`.
    let hindi = str::from_utf8(&s).unwrap();
    println!("The hindi : {}", hindi);
}
