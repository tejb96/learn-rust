Try for at least 30 minutes before reading this.

## Hint 1

Recursive fib works for small n in tests. Iterative version uses a loop with two accumulators.

---

## Hint 2

```rust
pub fn fib_iter(n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}
```

Wait — adjust base cases to match fib(0)=0, fib(1)=1.

---

## Hint 3

For recursive `fib`: standard definition. For iterative, loop `n` times updating `(prev, curr)`.

Run benchmarks optionally: `cargo bench -p benchmarks_lesson`.
