Try for at least 30 minutes before reading this.

## Hint 1

Iterator adapters: `.iter().filter().map().collect()`. Closures capture with `move`.

---

## Hint 2

`squares_above`: filter `*n > threshold`, map `n * n`. `make_adder`: `move |x| x + n`.

---

## Hint 3

```rust
pub fn partition_evens_odds(nums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let evens: Vec<_> = nums.iter().copied().filter(|n| n % 2 == 0).collect();
    let odds: Vec<_> = nums.iter().copied().filter(|n| n % 2 != 0).collect();
    (evens, odds)
}
```
