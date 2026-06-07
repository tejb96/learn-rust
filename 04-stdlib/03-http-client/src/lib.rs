#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

/// Parses a minimal HTTP response string: status line + blank line + body.
/// Expects first line like `HTTP/1.1 200 OK`.
pub fn parse_response(raw: &str) -> Result<HttpResponse, String> {
    Err(String::new())
}

/// Returns true when status is in the 2xx range.
pub fn is_success(resp: &HttpResponse) -> bool {
    false
}

/// Extracts JSON-like `"key":"value"` pairs from a simple body (no nested objects).
pub fn parse_simple_json(body: &str) -> Vec<(String, String)> {
    vec![]
}
