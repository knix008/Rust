#![allow(dead_code)]
#![allow(non_snake_case)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home : {}", home.address);
    println!("Home : {}", loopback.address);
}
