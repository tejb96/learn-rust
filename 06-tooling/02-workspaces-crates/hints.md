Try for at least 30 minutes before reading this.

## Hint 1

The `calc_core` crate is a path dependency. Import its functions with `use calc_core::{add, mul}`.

---

## Hint 2

`sum_then_mul`: `mul(add(a, b), c)`. `mul_then_add`: `add(mul(a, b), c)`.

---

## Hint 3

```rust
pub fn sum_then_mul(a: i32, b: i32, c: i32) -> i32 {
    mul(add(a, b), c)
}
```
