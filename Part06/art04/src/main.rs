use my_art::PrimaryColor;
use my_art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
    println!("Done!!!");
}
