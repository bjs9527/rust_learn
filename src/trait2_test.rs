pub trait Summer {
    fn summery(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summer for NewArticle {
    fn summery(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summer for Tweet {
    fn summery(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::trait2_test::Summer;

    use super::Tweet;

    #[test]
    fn test() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        assert_eq!(
            tweet.summery(),
            "horse_ebooks: of course, as you probably already know, people"
        );
    }
}
