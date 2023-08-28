fn bar() -> ! {
    println!("This function will not return any value!!!");
    panic!("Exit!!!");
}

fn main() {
    bar();
}
