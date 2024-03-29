#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::*;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value,
        }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = Guess::new(match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        });
        
        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
