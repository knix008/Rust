use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle1 = thread::spawn(move || {
        let mut num1 = counter.lock().unwrap();
        *num1 += 1;
    });
    handles.push(handle1);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();
        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
