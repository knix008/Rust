fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("Some number : {:?}", some_number);
    println!("Some number : {:?}", some_string);
    println!("Some number : {:?}", absent_number);
}
