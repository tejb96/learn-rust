pub trait Formatter {
    fn format(&self, input: &str) -> String;
}

pub struct Uppercase;
pub struct Prefix {
    pub tag: String,
}

impl Formatter for Uppercase {
    fn format(&self, input: &str) -> String {
        String::new()
    }
}

impl Formatter for Prefix {
    fn format(&self, input: &str) -> String {
        String::new()
    }
}

/// Applies each formatter in order, passing output of one as input to the next.
pub fn pipeline(input: &str, formatters: &[Box<dyn Formatter>]) -> String {
    String::new()
}
