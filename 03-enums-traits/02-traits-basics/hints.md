Try for at least 30 minutes before reading this.

## Hint 1

Traits define shared behavior. Default methods in the trait can be overridden in `impl`.

---

## Hint 2

`summarize` formats headline/body or username/content. Default `preview` can call `summarize` and truncate — Tweet overrides to append `"..."`.

---

## Hint 3

```rust
fn summarize(&self) -> String {
    format!("{}: {}", self.headline, self.body)
}

pub fn notify(item: &dyn Summary) -> String {
    format!("Breaking! {}", item.summarize())
}
```
