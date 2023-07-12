#![allow(non_snake_case)]
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("The scores : {:?}", scores);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("The score for Blue team : {:?}", score); // "score" is Option<T>
}