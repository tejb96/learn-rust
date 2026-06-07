Try for at least 30 minutes before reading this.

## Hint 1

Axum router: `Router::new().route("/health", get(health)).route("/items", get(list_items)).with_state(...)`.

---

## Hint 2

Check header with `headers.get("X-API-Key")`. Compare to `"dev-key"`. Return `401 UNAUTHORIZED` if missing/wrong.

---

## Hint 3

Lock state mutex, collect values into `Vec`, return `Ok(Json(vec))`.
