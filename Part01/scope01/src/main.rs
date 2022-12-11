fn main() {
    // s is not valid here; it's not yet declared
    let s = "Hello"; // s is valid from this point forward
                     // do stuff with s
    println!("{}", s);
} // this scope is now over, and s is no longer valid
