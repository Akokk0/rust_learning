use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        todo!()
    }

    /*fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }*/
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        todo!()
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(flag: bool) -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh, PA, USA")
    }

    // 这里如果确定了返回NewsArticle就只能返回这个，例如下面就会报错
    /*if flag {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
            author: String::from("Iceburgh"),
            location: String::from("Pittsburgh, PA, USA")
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false
        }
    }*/
}