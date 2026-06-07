Try for at least 30 minutes before reading this.

## Hint 1

`&mut T` is a mutable borrow — one mutable borrow at a time. Use `s.push_str(suffix)` to append.

---

## Hint 2

`longest` compares `a.len()` and `b.len()`. `double_in_place` uses a for loop over `nums.iter_mut()`.

---

## Hint 3

```rust
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() { b } else { a }
}

pub fn double_in_place(nums: &mut [i32]) {
    for n in nums.iter_mut() {
        *n *= 2;
    }
}
```
