pub trait Summary0 {
    fn summarize0(&self) -> String;
}

pub trait Summary1 {
    fn summarize1(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary0 for NewsArticle {
    fn summarize0(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary1 for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary0 for Tweet {
    fn summarize0(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary1 for Tweet {
    fn summarize1(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking new! {}", item.summarize());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
