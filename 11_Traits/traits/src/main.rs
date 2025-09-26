pub struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more...")
    }
}


fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("@divakar"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };

    let article: NewsArticle = NewsArticle {
        author: String::from("Divakar D"),
        headline: String::from("Learning Rust"),
        content: String::from("Learning Rust"),
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article Summary: {}", article.summarize());
}
