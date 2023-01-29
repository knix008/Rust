fn main() {
    let reference_to_nothing = dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
