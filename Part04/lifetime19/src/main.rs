fn print_string(s: &str) {
    println!("The string literal : {}.", s);
}

fn main() {
    let s: &'static str = "The s has a static lifetime.";
    print_string(s);
}
