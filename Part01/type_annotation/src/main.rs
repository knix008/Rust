fn main() {
    // with type annotation.
    //let guess: u32 = "42".parse().expect("Not a number!");
    // without type annotation.
    let guess = "42".parse().expect("Not a number!");
}
