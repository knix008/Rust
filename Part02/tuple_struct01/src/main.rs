fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 1, 2);
    let origin = Point(3, 4, 5);

    println!("{}", black.0);
    println!("{}", black.1);
    println!("{}", black.2);
    println!("{}", origin.0);
    println!("{}", origin.1);
    println!("{}", origin.2);
}
