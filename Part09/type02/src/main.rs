#![allow(unused_variables)]
#![allow(unused_must_use)]

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    println!("Take long type!!!");
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    let y: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("bye"));
    
    println!("Return long type!!!");
    y
}

fn main() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    takes_long_type(f);
    returns_long_type();
}
