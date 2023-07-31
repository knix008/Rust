fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    //let _ = v1.iter().map(|x| x + 1); /* To clear warning, you can use this line.*/
    v1.iter().map(|x| x + 1);

    println!("The new iterator : {:?}.", v1);
}
