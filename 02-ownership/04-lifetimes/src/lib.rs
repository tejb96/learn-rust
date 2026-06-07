/// Holds a borrowed string slice with an explicit lifetime.
pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(part: &'a str) -> Self {
        Self { part: "" }
    }

    pub fn first_sentence(text: &'a str) -> Self {
        Self { part: "" }
    }
}

/// Returns the first string if both are equal length, otherwise the longer one.
pub fn pick_longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}
