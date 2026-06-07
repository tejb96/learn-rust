pub trait Summary {
    fn summarize(&self) -> String;

    fn preview(&self) -> String {
        String::new()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub body: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        String::new()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        String::new()
    }

    fn preview(&self) -> String {
        String::new()
    }
}

pub fn notify(item: &dyn Summary) -> String {
    String::new()
}
