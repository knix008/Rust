#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("The IP Address : {:?}", home);
}
