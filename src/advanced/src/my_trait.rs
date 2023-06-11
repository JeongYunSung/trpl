trait Summary {
    fn summarize(&self) -> String;

    fn default(&self) -> String {
        self.summarize()
    }
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

    fn default(&self) -> String {
        let mut string = String::from("Override : ");
        string.push_str(self.summarize().as_str());
        string
    }
}

impl Summary for Vec<i32> {
    fn summarize(&self) -> String {
        format!("Vec<int32> length: {}", self.len())
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn create_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub fn print() {
    print_summary(&create_tweet());
    print_summary(&vec![1, 2, 3, 4, 5]);
}