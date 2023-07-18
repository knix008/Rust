#![allow(unused_variables)]
#![allow(non_snake_case)]

use notifySummary07::Summary;
use notifySummary07::NewsArticle;
use notifySummary07::Tweet;

pub fn notify<T>(item: T) where T: Summary {
    println!("The item : {}.", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
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

    let tweet = returns_summarizable();
    notify(tweet);
}