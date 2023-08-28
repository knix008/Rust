#![allow(unused_variables)]
#![allow(unused_must_use)]

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    println!("Take long type!!!");
}

fn returns_long_type() -> Thunk {
    let y: Thunk = Box::new(|| println!("bye"));

    println!("Return long type!!!");
    y
}

fn main() {
    let f: Thunk = Box::new(|| println!("hi"));

    takes_long_type(f);
    returns_long_type();
}
