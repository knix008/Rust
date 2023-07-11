#![allow(non_snake_case)]

fn main() {
    let hindi = "नमस्ते";

    for b in hindi.bytes() {
        println!("{}", b);
    }
}
