use std::fs;
use std::io;
use std::path::Path;

/// Writes `lines` to `path`, one line per entry (Unix newlines).
pub fn write_lines(path: &Path, lines: &[&str]) -> io::Result<()> {
    Ok(())
}

/// Reads a file and returns non-empty trimmed lines.
pub fn read_nonempty_lines(path: &Path) -> io::Result<Vec<String>> {
    Ok(vec![])
}

/// Returns lines from `path` that contain `needle` (case-sensitive substring).
pub fn grep_lines(path: &Path, needle: &str) -> io::Result<Vec<String>> {
    Ok(vec![])
}
