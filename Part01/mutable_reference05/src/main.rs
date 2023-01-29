fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{}, {}", r1, r2);
    let r3 = &mut s; // NO PROBLEM
    r3.push_str(" world!!!");
    println!("{}", r3); 
}
