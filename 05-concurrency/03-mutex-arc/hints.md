Try for at least 30 minutes before reading this.

## Hint 1

Wrap counter in `Arc<Mutex<i32>>`. Clone `Arc` for each thread. Lock mutex before mutating.

---

## Hint 2

Each thread: `*counter.lock().unwrap() += 1`. Join all handles before reading final value.

---

## Hint 3

For `parallel_insert`, value is always `1`. Use `Arc<Mutex<HashMap<...>>>` similarly.
