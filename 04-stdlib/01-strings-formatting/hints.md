Try for at least 30 minutes before reading this.

## Hint 1

Implement `Display` with `write!(f, "...")`. Use `format!` macro for owned strings.

---

## Hint 2

`join_words`: handle empty slice; otherwise use `words.join(", ")` or manual iteration.

---

## Hint 3

```rust
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}
```
