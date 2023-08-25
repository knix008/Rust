use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

//#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

fn main() {
    let x = Point{x: 1, y: 3};
    println!("The x : {:?}.", x);
}