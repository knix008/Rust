fn print_something<T: Sized + std::fmt::Display>(value: T) {
    println!("{}", value)
}

fn main() {
    print_something("Hello, world!");
    print_something(100);
    print_something(0.1);
}
