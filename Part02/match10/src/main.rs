#![allow(dead_code)]
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --skip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => {
            count += 1;
        }
    }
    println!("The others : {}", count);
}
