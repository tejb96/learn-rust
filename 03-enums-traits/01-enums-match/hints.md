Try for at least 30 minutes before reading this.

## Hint 1

Use `match` on the enum, binding struct fields with `Message::Move { x, y }`.

---

## Hint 2

Each variant gets a distinct string in `describe`. `apply_move` only changes position for `Move`.

---

## Hint 3

Match all four variants in `describe`. For color use format `"color ({r}, {g}, {b})"`.
