#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::fmt::Display;
use notifySummary05::Summary;
use notifySummary05::NewsArticle;
use notifySummary05::Tweet;

pub fn notify<T: Summary + Display>(item: T) {
    println!("The item : {}.", item.summarize());
    println!("The item : {}.", item );
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL."
        ),
    };
    notify(article);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(tweet);
}
