use art008::PrimaryColor;
use art008::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
    println!("Done!!!");
}
