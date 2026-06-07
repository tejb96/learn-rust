Try for at least 30 minutes before reading this.

## Hint 1

`Option<T>` is Rust's nullable replacement — `Some(value)` or `None`. Use `s.parse::<u16>()` which returns `Result`; convert with `.ok()`.

---

## Hint 2

For `first_even`, iterate and return `Some(n)` on the first `n % 2 == 0`. For `safe_divide`, guard on `b == 0.0`.

---

## Hint 3

```rust
pub fn parse_port(s: &str) -> Option<u16> {
    if s.is_empty() {
        return None;
    }
    s.parse().ok()
}

pub fn first_even(nums: &[i32]) -> Option<i32> {
    nums.iter().copied().find(|n| n % 2 == 0)
}
```
