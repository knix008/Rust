fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("R1 : {:?}", r1);
    println!("R2 : {:?}", r2);
}
