#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    //p1.distance(&p2);
    //(&p1).distance(&p2);
    println!("1. The area of the rectangle is {} square pixels.", rect1.area());
    println!("2. The area of the rectangle is {} square pixels.", (&rect1).area());
}
