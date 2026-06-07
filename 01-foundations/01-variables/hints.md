Try for at least 30 minutes before reading this.

## Hint 1

Rust gives every type a **default value** via `Default::default()` or type-specific defaults. For `String` that is `""`, for `int` it is `0`. Tuples let you return multiple values like Go's multiple return values.

---

## Hint 2

For `swap`, return `(b.to_string(), a.to_string())`. For `describe_default`, use a `match` on `kind` and return `Err(...)` for unknown kinds. Compare empty string with `name.is_empty()`.

---

## Hint 3

```rust
pub fn swap(a: &str, b: &str) -> (String, String) {
    (b.to_string(), a.to_string())
}

pub fn default_greeting(name: &str) -> String {
    if name.is_empty() {
        "Hello, stranger!".to_string()
    } else {
        format!("Hello, {name}!")
    }
}
```

For `describe_default`, map each kind to the exact strings the tests expect.
