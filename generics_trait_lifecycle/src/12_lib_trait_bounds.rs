use std::fmt::Display;

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

// Trait bound 语法糖
/*pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}*/

// Trait bound
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

// Trait bound 写法好处
/*pub fn notify1(item1: impl Summary, item2: impl Summary) {
    println!("Breaking news! {}", item1.summarize())
}

pub fn notify2<T: Summary>(item1: T, item2: T) {
    println!("Breking news! {}", item1.summarize())
}*/

// 指定多个Trait，同时约束
pub fn notify1(item1: impl Summary + Display) {
    println!("Breaking news! {}", item1.summarize())
}

pub fn notify2<T: Summary + Display>(item1: T) {
    println!("Breking news! {}", item1.summarize())
}