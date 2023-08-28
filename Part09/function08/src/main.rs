fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let ret = returns_closure();
    println!("The returned closure : {}.", ret(5))
}
