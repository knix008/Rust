struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("Feet : {}", feet);
    println!("Inch : {}", inches);
    println!("X : {}", x);
    println!("X : {}", y);
}
