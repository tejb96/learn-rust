use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
}

/// Parses a line in the form `"LEVEL message text"`.
/// LEVEL is the first field; the rest is the message.
pub fn parse_line(line: &str) -> Result<LogEntry, String> {
    Err(String::new())
}

/// Returns entries matching `level` (case-insensitive). Empty level returns all.
pub fn filter_by_level(entries: &[LogEntry], level: &str) -> Vec<LogEntry> {
    vec![]
}

/// Formats entries as JSON array string.
pub fn format_json(entries: &[LogEntry]) -> Result<String, serde_json::Error> {
    Ok(String::new())
}
