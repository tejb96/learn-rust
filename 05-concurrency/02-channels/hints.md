Try for at least 30 minutes before reading this.

## Hint 1

`mpsc::channel()` returns `(Sender, Receiver)`. Producer sends; main thread receives until channel closes.

---

## Hint 2

Drop the sender when done sending so receiver loop ends. Collect into `Vec` in receive order.

---

## Hint 3

Spawn thread with cloned sender or move sender after sending all values; drop sender; recv in loop.
