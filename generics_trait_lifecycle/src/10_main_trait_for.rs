use generics_trait_lifecycle::Summary; // items from traits can only be used if the trait is in scope
use generics_trait_lifecycle::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize())
}