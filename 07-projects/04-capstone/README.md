# Capstone Projects

You have completed the structured lessons. These open-ended projects apply what you learned to **systems and DevOps** tooling. There are no tests — you define success criteria.

Try for at least a few hours on one idea before jumping to the next.

---

## 1. Kubernetes pod health checker

Build a CLI that:

- Accepts a kubeconfig path and namespace
- Lists pods and reports which are not `Running` / not ready
- Outputs JSON or a table for CI pipelines

**Stretch goals:** watch mode, exit code non-zero when unhealthy pods exist, timeout per API call.

**Skills used:** CLI parsing, JSON output, HTTP client, error handling.

---

## 2. Infrastructure plan summary parser

Build a tool that:

- Reads JSON plan output from stdin or a file
- Summarizes resources to **create**, **update**, **delete**
- Prints a one-screen summary for PR comments

**Stretch goals:** filter by resource type, fail if delete count exceeds threshold.

**Skills used:** serde JSON, structs, file/stdin input, formatting.

---

## 3. Metrics sidecar / log forwarder

Build a small HTTP service that:

- Exposes `GET /metrics` in Prometheus text format
- Accepts `POST /events` with JSON log lines and buffers them
- Flushes buffer to stdout or a file every N seconds or on shutdown

**Stretch goals:** graceful shutdown, worker pool for flush, integration tests with axum.

**Skills used:** axum/tower, concurrency, channels, signal handling.

---

## Before you call it done

For whichever project you choose:

- [ ] `cargo test` covers core logic (even if you design the tests yourself)
- [ ] README with build/run instructions
- [ ] Error handling on I/O and network paths
- [ ] No hard-coded secrets — env vars or flags

## Where to go next

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- Share your capstone repo or contribute back to this course

Congratulations on finishing the course modules.
