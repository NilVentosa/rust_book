fn main() {
    let tweet = Tweet {
        username: String::from("NilVS"),
        content: String::from("oiahjsdoijsado asodijaosijdoa asdoijasoijdas"),
        reply: false,
        retweet: false,
    };

    let post = BlogPost {
        author: String::from("Nil Ventosa"),
        content: String::from("The content..."),
        title: String::from("hello there..."),
    };

    println!("{}", tweet.summarize());

    println!("{}", post.summarize());

    notify(&post);
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
}

pub struct BlogPost {
    pub author: String,
    pub content: String,
    pub title: String,
}

impl Summary for BlogPost {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
