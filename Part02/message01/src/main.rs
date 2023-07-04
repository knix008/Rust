#![allow(dead_code)]
#![allow(non_snake_case)]
#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let message = Message::Quit;

    println!("Message : {:?}", message);
    
}
