fn main() {
    let mut i = 0;

    loop {
        println!("again!");
        i = i + 1;
        if i == 3 {
            return;
        }
    }
}
