use std::fmt;

pub struct Person {
    pub name: String,
    pub age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/// Formats `"NAME (age N)"` using `format!`.
pub fn format_person(name: &str, age: u32) -> String {
    String::new()
}

/// Joins words with commas, no trailing comma.
pub fn join_words(words: &[&str]) -> String {
    String::new()
}
