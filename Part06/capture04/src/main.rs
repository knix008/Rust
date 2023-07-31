fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    //println!("can't use x here: {:?}", x);
    println!("The result of closure : {}.", equal_to_x(vec![1, 2, 3]));

    let y = vec![1, 2, 3];
    
    assert!(equal_to_x(y));
    println!("Done!!!");
}
