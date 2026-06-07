Try for at least 30 minutes before reading this.

## Hint 1

Split on `\r\n\r\n` or `\n\n` for headers vs body. Parse status code from first line splitting on spaces.

---

## Hint 2

Status is second token in `HTTP/1.1 200 OK`. `is_success`: `status >= 200 && status < 300`.

---

## Hint 3

For simple JSON, strip `{`, `}`, split on `,`, then split each pair on `:` and trim quotes from keys/values.
