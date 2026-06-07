Try for at least 30 minutes before reading this.

## Hint 1

Each job `i` produces output `i * 2`. Use channels to send `(index, result)` back to main.

---

## Hint 2

Spawn `workers` threads consuming jobs from a queue. Main submits `n` jobs then collects results.

---

## Hint 3

Sort results by index before returning. Drop job sender and join workers on shutdown.
