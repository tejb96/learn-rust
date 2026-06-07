Try for at least 30 minutes before reading this.

## Hint 1

`impl Rectangle` blocks hold methods. `&self` is immutable borrow; `&mut self` allows mutation.

---

## Hint 2

`area` returns `self.width * self.height`. `is_square` compares width and height. `scale` multiplies both fields by `factor`.

---

## Hint 3

```rust
pub fn area(&self) -> u32 {
    self.width * self.height
}

pub fn scale(&mut self, factor: u32) {
    self.width *= factor;
    self.height *= factor;
}
```
