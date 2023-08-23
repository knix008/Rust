unsafe trait Foo {
    // methods go here
    fn msg();
}

unsafe impl Foo for i32 {
    // method implementations go here
    fn msg() {
        println!("I am unsafe trait implementation for i32.");
    }
}

fn main() {
    i32::msg();
}
