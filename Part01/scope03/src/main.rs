fn scope() {
    let s = String::from("hello"); // s is valid from this point forward
                                   // do stuff with s
    println!("{}", s);
} // this scope is

fn main() {
    scope();
    println!("End of the main function.");
}
