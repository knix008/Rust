use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u64) -> u64 {
    println!("calculating slowly... {} seconds", intensity);
    thread::sleep(Duration::from_secs(intensity));
    intensity
}

fn main() {
    println!("The intensity : {} seconds.", simulated_expensive_calculation(3));
}
