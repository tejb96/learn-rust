Try for at least 30 minutes before reading this.

## Hint 1

Rust 1.63+ has `thread::scope` for spawning threads that borrow local data.

---

## Hint 2

Split `nums` mutably into chunks, spawn scoped threads that mutate each chunk.

---

## Hint 3

```rust
thread::scope(|s| {
    for chunk in nums.chunks_mut(2) {
        s.spawn(|| {
            for n in chunk.iter_mut() {
                *n *= 2;
            }
        });
    }
});
```
