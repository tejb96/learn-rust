Try for at least 30 minutes before reading this.

## Hint 1

Use `#[cfg(feature = "enabled")]` on alternate function implementations or within functions.

---

## Hint 2

You can use two impl blocks or conditional compilation inside each function returning different values.

---

## Hint 3

```rust
pub fn bonus_value() -> i32 {
    #[cfg(feature = "enabled")]
    { return 42; }
    #[cfg(not(feature = "enabled"))]
    { 0 }
}
```

Run gated tests: `cargo test --features enabled`
