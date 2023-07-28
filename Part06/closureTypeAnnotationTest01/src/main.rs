#![allow(non_snake_case)]

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;
    
    println!("Closure Type Annotation & Inference Test!!!");
    println!("The result of V1 : {}.", add_one_v1(1));
    println!("The result of V2 : {}.", add_one_v2(2));
    println!("The result of V3 : {}.", add_one_v3(3));
    println!("The result of V4 : {}.", add_one_v4(4));
}
