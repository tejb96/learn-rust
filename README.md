# Rust Learning Course

A self-contained, test-driven Rust course. Each lesson gives you reading material, failing tests, and progressive hints. You learn by making tests pass.

## Prerequisites

- [Rust stable](https://rustup.rs/) installed (`rustc --version` should report a recent stable release)
- Basic terminal comfort (`cd`, `ls`, running commands)
- A text editor or IDE with rust-analyzer recommended

## Course map

| Module | Topics |
|--------|--------|
| **01-foundations** | Variables, functions, Result/Option, Vec/HashMap, structs |
| **02-ownership** | Move semantics, borrowing, slices, lifetimes, smart pointers |
| **03-enums-traits** | Enums/match, traits, trait objects, generics, custom errors |
| **04-stdlib** | Strings, file I/O, HTTP parsing, JSON, iterators |
| **05-concurrency** | Threads, channels, Mutex/Arc, scoped threads, patterns |
| **06-tooling** | Testing, workspaces, benchmarks, cfg features |
| **07-projects** | CLI log parser, REST API, worker pool, capstone ideas |

## Quick start

Pick a lesson and run its tests:

```bash
cd 01-foundations/01-variables
cargo test
```

Tests fail on day one — that is expected. Open `README.md` in the same folder, read the concept, then edit `src/lib.rs` until tests pass.

## Running tests

Each lesson is an independent crate in the workspace. Run tests from inside the lesson directory:

```bash
cd 01-foundations/01-variables
cargo test
```

Run all lessons from the repo root:

```bash
cargo test --workspace
```

Run every lesson in a module with a loop:

```bash
for d in 01-foundations/*/; do (cd "$d" && cargo test) || exit 1; done
for d in 02-ownership/*/; do (cd "$d" && cargo test) || exit 1; done
```

Or use the check script for a pass/fail summary:

```bash
chmod +x scripts/check-all.sh
./scripts/check-all.sh
```

## Feature-gated lesson

The build-flags lesson requires an extra feature flag:

```bash
cd 06-tooling/04-cfg-features
cargo test --features enabled
```

## Progression rules

1. Start at `01-foundations/01-variables` and work in order within each module.
2. **Do not advance** until every test in the current lesson passes (`cargo test` exits 0).
3. Read the lesson README fully before coding.
4. Do not edit `tests/exercise.rs` — implement everything in `src/lib.rs`.

## Using hints without cheating yourself

Each lesson has a `hints.md` with three progressive hints separated by horizontal rules.

- Try for **at least 30 minutes** before opening hints.
- Read **only the first hint** initially. Implement, re-run tests, think again.
- Open the next hint only when you are truly stuck — not when you want a shortcut.
- The third hint is nearly a solution. Using it teaches you less. That is fine occasionally, but do not make it a habit.

## What each lesson contains

```
01-foundations/01-variables/
  README.md           # concept, examples, checklist
  tests/exercise.rs   # DO NOT EDIT — your spec
  hints.md            # unlockable help
  src/lib.rs          # YOU implement here
  Cargo.toml
```

Physical directories use numeric prefixes (`01-variables`); **crate names** drop the prefix (`variables`).

## Success criterion

A lesson is complete when `cargo test` exits with code **0**. Any failing assertion or compile error means keep working (or read hints).
