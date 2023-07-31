fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1
        .iter()
        .map(|x| x + 1)
        .collect();

    assert_eq!(v2, vec![2, 3, 4]);
    println!("The v2 is : {:?}.", v2);
}
