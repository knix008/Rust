#![allow(non_snake_case)]
#![allow(dead_code)]
use std::cmp::Ordering;

#[derive(Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

fn main() {
    let person01 = Person { id: 01, name: "Mark".to_string(), height: 184 };
    let person02 = Person { id: 02, name: "Steve".to_string(), height: 190 };

    if person01 > person02 {
        println!("{} is taller than {}.", person01.name, person02.name);
    } else {
        println!("{} is taller than {}.", person01.name, person02.name);
    }
}
