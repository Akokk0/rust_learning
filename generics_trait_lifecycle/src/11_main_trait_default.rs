use generics_trait_lifecycle::Summary; // items from traits can only be used if the trait is in scope
use generics_trait_lifecycle::NewsArticle;

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh, PA, USA")
    };

    println!("1 new tweet: {}", article.summarize())
}