#![allow(dead_code)]
#![allow(non_snake_case)]

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let i;
    {
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt { part: first_sentence };
    }
    println!("The part : {:?}.", i);
}
