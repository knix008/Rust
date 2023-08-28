#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("The list of statuses : {:?}", list_of_statuses);
}
