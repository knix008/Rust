#![allow(unused_variables)]
#![allow(non_snake_case)]

use makeSummary02::Summary;
use makeSummary02::NewsArticle;
use makeSummary02::Tweet;

fn main() {
    let news = NewsArticle {
        headline: "News".to_string(),
        location: "Seoul".to_string(),
        author: "shkwon".to_string(),
        content: "Nothing".to_string(),
    };
    println!("The summary : {}.", news.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
