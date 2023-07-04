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

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("The Messsage Write : {:?}", self);
    }
}

fn main() {
    let message = Message::Write(String::from("hello"));
    message.call();
}
