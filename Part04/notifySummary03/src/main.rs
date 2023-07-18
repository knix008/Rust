#![allow(unused_variables)]
#![allow(non_snake_case)]

use notifySummary03::Summary;
use notifySummary03::NewsArticle;
use notifySummary03::Tweet;

pub fn notify(item1: impl Summary, item2: impl Summary) {
    println!("The article : {}.", item1.summarize());
    println!("The tweet: {}.", item2.summarize());
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

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(article, tweet);
}