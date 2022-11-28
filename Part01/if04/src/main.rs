fn main() {
    let condition = true;
    //let number = if condition { 5 } else { 6 };
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
