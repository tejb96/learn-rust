/// Returns absolute value. BUG: fails for i32::MIN.
pub fn abs_value(n: i32) -> i32 {
    if n < 0 {
        -n
    } else {
        n
    }
}

/// Divides a by b. BUG: uses floating point logic incorrectly for integers.
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Parses bool from string. BUG: accepts "YES" but should only accept "true"/"false".
pub fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "true" | "YES" => Some(true),
        "false" | "NO" => Some(false),
        _ => None,
    }
}
