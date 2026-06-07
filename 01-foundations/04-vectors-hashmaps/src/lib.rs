use std::collections::HashMap;

/// Returns words longer than `min_len` characters, preserving order of first appearance.
pub fn filter_long_words(words: &[&str], min_len: usize) -> Vec<String> {
    vec![]
}

/// Counts occurrences of each word (case-sensitive).
pub fn word_counts(words: &[&str]) -> HashMap<String, usize> {
    HashMap::new()
}

/// Returns the key with the highest count. Ties: lexicographically smallest key wins.
pub fn top_key(counts: &HashMap<String, usize>) -> Option<String> {
    None
}
