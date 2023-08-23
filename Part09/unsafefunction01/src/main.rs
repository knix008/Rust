unsafe fn dangerous() {}

fn main() {
    unsafe {
        dangerous();
    }
    println!("Done!!!");
}
