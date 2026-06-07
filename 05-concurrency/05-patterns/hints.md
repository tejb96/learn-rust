Try for at least 30 minutes before reading this.

## Hint 1

Worker pool: channel of jobs, N worker threads loop on `receiver.recv()`, main sends jobs.

---

## Hint 2

Use a result channel or collect handles. For `run_jobs`, submit closures that compute `f(i)` and collect in order.

---

## Hint 3

Simplest approach: spawn `workers` threads sharing job queue; use another channel for `(index, result)` pairs; sort by index.
