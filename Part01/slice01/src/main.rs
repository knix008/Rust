fn main() {
    let s = String::from("Hello, world!!!");
    let size = first_word(&s);
    println!("The size of first word : {}", size)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
