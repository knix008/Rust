#![allow(non_snake_case)]

fn main() {
    {
        let v = vec![1, 2, 3, 4];
        
        println!("Vector : {:?}", v);
    }
    //println!("Creating Vector : {:?}", v); // This line will make a compile error!!!
    // <- v goes out of scope and is freed here
}
