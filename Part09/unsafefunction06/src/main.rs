use std::slice;

fn main() {
    let address = 0x012345usize;
    let r = address as *mut i32;
    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000);
    };
    println!("Done!!!");
}
