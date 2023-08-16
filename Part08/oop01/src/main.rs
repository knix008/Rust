#![allow(dead_code)]
#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

fn main() {
    let avc = AveragedCollection{list: vec![], average: 0.0};
    println!("The avc : {:?}.", avc);
}
