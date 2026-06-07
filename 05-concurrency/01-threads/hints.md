Try for at least 30 minutes before reading this.

## Hint 1

Use `thread::spawn` and `.join().unwrap()` to wait for completion. Move data into the closure.

---

## Hint 2

For `parallel_sum`, split slice at midpoint, spawn two threads, sum each half.

---

## Hint 3

```rust
let handle = thread::spawn(move || {
    nums.into_iter().map(|n| n * 2).collect::<Vec<_>>()
});
handle.join().unwrap()
```
