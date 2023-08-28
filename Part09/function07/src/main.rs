fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let ret = returns_closure();
    println!("The returned closure : {}.", ret(5))
}
