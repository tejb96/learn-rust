// DO NOT EDIT — implement the solution in src/lib.rs

use traits_basics::{notify, NewsArticle, Summary, Tweet};

#[test]
fn news_summarize() {
    let article = NewsArticle {
        headline: "Rust 2024".into(),
        body: "Great year.".into(),
    };
    assert_eq!(article.summarize(), "Rust 2024: Great year.");
}

#[test]
fn tweet_summarize() {
    let tweet = Tweet {
        username: "ferris".into(),
        content: "learning traits".into(),
    };
    assert_eq!(tweet.summarize(), "ferris: learning traits");
}

#[test]
fn tweet_preview() {
    let tweet = Tweet {
        username: "ferris".into(),
        content: "hello".into(),
    };
    assert_eq!(tweet.preview(), "hello...");
}

#[test]
fn notify_uses_summarize() {
    let tweet = Tweet {
        username: "a".into(),
        content: "b".into(),
    };
    assert_eq!(notify(&tweet), "Breaking! a: b");
}
