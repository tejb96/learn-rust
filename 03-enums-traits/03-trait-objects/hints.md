Try for at least 30 minutes before reading this.

## Hint 1

`Box<dyn Formatter>` stores trait objects on the heap. Each formatter has different behavior at runtime.

---

## Hint 2

Loop over formatters, updating a `mut result` string. Start with `input.to_string()`.

---

## Hint 3

```rust
pub fn pipeline(input: &str, formatters: &[Box<dyn Formatter>]) -> String {
    let mut result = input.to_string();
    for f in formatters {
        result = f.format(&result);
    }
    result
}
```
