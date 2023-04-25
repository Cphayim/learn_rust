use std::fmt::{Debug, Display};

use trait_demo::{Summary, Tweet};

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);

    notify2(&tweet);

    let a = returns_summarizable();
}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    123
}

pub fn some_function2<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    123
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}
