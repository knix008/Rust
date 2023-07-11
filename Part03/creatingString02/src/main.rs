#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

fn main() {
     let mut s= String::new();
     println!("The string : {}", s);

     let data = "initial contents";
     let s = data.to_string();
     println!("1. The string : {}", s);

     // the method also works on a literal directly:
     let s = "initial contents".to_string();
     println!("2. The string : {}", s);

     let s = String::from("initial contents");
     println!("3. The string : {}", s);
}