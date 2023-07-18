#![allow(unused_variables)]
#![allow(non_snake_case)]

use notifySummary01::Summary;
use notifySummary01::NewsArticle;
use notifySummary01::Tweet;

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
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
