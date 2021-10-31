pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("maxfer1221"),
        content: String::from("this is my first tweet!"),
        reply: false,
        retweet: false
    };

    println!("{}", tweet.summarize());
}
