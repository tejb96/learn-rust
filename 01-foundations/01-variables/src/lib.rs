/// Swap returns `(b, a)` — exchanging two string values.
pub fn swap(a: &str, b: &str) -> (String, String) {
    (String::new(), String::new())
}

/// Returns a short description of the default value for the given kind.
/// `kind` must be one of: `"string"`, `"int"`, `"bool"`, `"slice"`.
pub fn describe_default(kind: &str) -> Result<String, String> {
    Ok(String::new())
}

/// Returns a greeting for `name`. When name is empty, return `"Hello, stranger!"`.
pub fn default_greeting(name: &str) -> String {
    String::new()
}
