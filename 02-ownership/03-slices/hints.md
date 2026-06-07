Try for at least 30 minutes before reading this.

## Hint 1

Slice syntax: `&data[start..end]`. Check bounds before slicing.

---

## Hint 2

For `first_word`, find `s.find(' ')` and slice up to that index, or return `s` if None.

---

## Hint 3

```rust
pub fn safe_slice(data: &[i32], start: usize, end: usize) -> Option<&[i32]> {
    if start <= end && end <= data.len() {
        Some(&data[start..end])
    } else {
        None
    }
}
```
