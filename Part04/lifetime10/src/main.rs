#![allow(non_snake_case)]
#![allow(unused_variables)]

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt { part: first_sentence };
    let level = excerpt.level();

    println!("The excerpt : {:?}.", excerpt);
    println!("The level : {}.", level);

    let announce = excerpt.announce_and_return_part(&first_sentence);
    println!("The announce : {}.", announce);
}
