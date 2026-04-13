# 🦀 Rust Learning Notes

This document captures key Rust concepts learned while building a trading backtesting engine. The focus is on _why_ Rust works the way it does, not just syntax.

## Contents

1. [Ownership and borrowing](#1-ownership-and-borrowing)
2. [Option and representing absence](#2-option-and-representing-absence)
3. [Strategy logic: state and signals](#3-strategy-logic-state-and-signals)
4. [Backtesting engine patterns](#4-backtesting-engine-patterns)
5. [Testing in Rust](#5-testing-in-rust)
6. [Takeaways](#6-takeaways)
7. [Next topics](#7-next-topics-to-explore)

---

## 1. Ownership and borrowing

Rust enforces memory safety through **ownership**.

**Core ideas**

- Each value has one owner.
- Data can be **borrowed** (`&T`) instead of moved or copied.

**Example from this project**

```rust
for candle in &data {
```

Iterating over `&data` borrows the slice so the caller keeps ownership of `Vec<Candle>` (or `&[Candle]`) and can reuse it—for example, running several strategies on the same candles without cloning.

**Why it matters**

If you moved `data` into a loop without borrowing, you could not use it again. The compiler forces an explicit data flow, which shows up everywhere in the engine and in `run(data: &[Candle], ...)`.

---

## 2. Option and representing absence

Rust has no `null`. Absence is modeled with **`Option<T>`**:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**Why it exists**

You must decide what to do when a value might be missing—no silent `null` dereferences.

**Basic usage**

```rust
let value: Option<f64> = Some(10.5);
let empty: Option<f64> = None;
```

**`if let` — run code only when there is a value**

```rust
if let Some(v) = value {
    println!("Value is {}", v);
}
```

**`match` — handle every case**

```rust
match value {
    Some(v) => println!("Value is {}", v),
    None => println!("No value"),
}
```

`if let` is shorter when you only care about `Some`; `match` is better when every branch matters.

**Multiple options**

```rust
if let (Some(a), Some(b)) = (x, y) {
    // both exist
}
```

Used in the moving-average strategy so signals are only emitted when **both** previous short and long averages exist.

**`unwrap()` — use with care**

```rust
let v = value.unwrap();
```

This returns the inner value or **panics** if the option is `None`.

- Fine for quick experiments or when failure is truly impossible.
- Avoid in production, user-facing code, or trading logic—silent crashes are worse than compile-time or explicit errors.

**Strategy fields**

```rust
previous_short: Option<f64>
previous_long: Option<f64>
```

These start as `None` until enough history exists. Rust forces handling that case before computing crossover signals.

---

## 3. Strategy logic: state and signals

**State over time**

Strategies are not stateless formulas—they **remember** prior values (e.g. previous moving averages) so the program can detect **change**, not just level.

**Signals vs raw conditions**

A common mistake:

```text
If short > long → BUY
```

The crossover strategy needs **events**:

```text
If short crosses above long → BUY
```

That requires past and present values and comparing transitions, not a single snapshot.

Rust makes that state explicit in structs and `Option` fields, which matches how trading logic actually works.

---

## 4. Backtesting engine patterns

The following applies to `src/engine.rs`. Earlier sections cover `Option` in general (e.g. moving-average history); here the **engine** applies the same ideas to **positions**, **cash**, and **outputs**.

### `Option` for open positions

**What:** `position: Option<OpenPosition>` is either flat (`None`) or in a trade (`Some(...)`).

**How:** Starts as `None`. A fill sets `position = Some(OpenPosition { entry_price, size, allocation, buy_fee })`. Buys use `position.is_none()`; after a sell the slot is cleared (see `take()` below).

**Why:** Avoids sentinel values (e.g. `size == 0.0`) and matches the mental model: you either hold something or you do not.

### `Option::take()`

**What:** Moves `Some(x)` out, returns `x`, and sets the option to `None` in one step.

**How:** On a sell, `position.take()` yields the **`OpenPosition`** and clears the slot. `open_trade_id.take()` does the same for the id stored at buy time.

**Why:** Reduces “read, use, forget to clear” bugs and keeps duplicate state in sync.

### Floating-point comparisons

**What:** `f64` is approximate; `==` / `!=` are usually wrong for “is this zero?” or “are these equal?”

**How:** Before opening a trade, require meaningful size with something like `allocation > f64::EPSILON` in the buy branch—not `== 0.0` on floats for control flow.

**Why:** Long backtests compound rounding noise; a threshold stays stable for “enough cash to size a position?”

### Storing `allocation` and `buy_fee` on the position

**What:** Keep the **cash allocated** to the trade and the **entry fee** on **`OpenPosition`** (along with `entry_price` and `size`).

**How:** Set `allocation` and `buy_fee` at buy. On exit, PnL uses **net sell proceeds** minus **`allocation`** minus **`buy_fee`** (see `engine.rs`), so fees stay consistent in metrics and trade rows.

**Why:** One source of truth for capital at risk and entry costs keeps PnL, logs, and CSV exports aligned when **`fee_rate`** is non-zero.

### Passing `&[Candle]` instead of `Vec<Candle>`

**What:** The engine takes a **slice** `&[Candle]` so callers lend data without transferring ownership.

**How:** `fn run(data: &[Candle], ...)` and `main` passes `&candles` for each strategy run.

**Why:** No unnecessary clones, clearer intent, and slightly better performance on large datasets.

### Generic `run` with the `Strategy` trait

**What:** `fn run<S: Strategy>(...)` accepts any type that implements `Strategy`.

**How:** `MovingAverage`, `RandomStrategy`, and future strategies share one execution path.

**Why:** The engine stays decoupled from a single strategy implementation and stays easy to extend.

### Separation of concerns

**What:** Different modules own different jobs.

**How (this project):** strategy code emits **signals**; the **engine** applies fills and accounting; **CSV** writes files.

**Why:** Easier to test, extend, and reason about than one giant module.


---

## 5. Testing in Rust

Rust has built-in support for tests via the `#[test]` attribute.

Tests can live:

- inline with the code (unit tests)
- in a separate `tests/` directory (integration tests)

### Unit tests (inline)

**What:** Tests inside the same file, usually under:

```rust
#[cfg(test)]
mod tests {
    use super::*;
}
```

**How this project uses it:**

- `engine` tests for capital and PnL helpers
- `strategy` tests for signal behaviour (`moving_average`, `random`)

**Why it matters:**

- Can test private functions directly
- Keeps tests close to logic
- Encourages small, focused behaviour checks

### Integration tests (concept)

**What:** Tests in `tests/` such as `tests/engine_tests.rs`.

**Key difference:** They only use public APIs, not private internals.

**When to use:**

- Full workflows
- Cross-module interactions
- End-to-end behaviour validation

### Testing floating-point values

**Problem:** `f64` values should not be compared with `==` in tests.

**How this project handles it:** A small helper like `assert_close(a, b)` with tolerance.

**Why it matters:** Avoids flaky tests caused by precision noise.

### Testing strategy behaviour

**Key idea:** Test behaviour, not profitability.

- `MovingAverage`: hold before enough data, then emit valid crossover signals
- `RandomStrategy`: always returns a valid `Signal`; tests should not assert exact random sequences

This keeps strategy tests stable and meaningful.

### Separation of concerns in tests

- Engine tests: capital and PnL logic
- Strategy tests: signal generation
- Avoid coupling unit tests to CSV/file I/O or full loop integration

This keeps tests fast, deterministic, and easy to debug.

### Deterministic testing

- Use fixed numeric inputs for calculation tests
- Avoid randomness in assertions
- For random strategies, verify output validity and no panic, not exact outcomes


---

## 6. Takeaways

- **Ownership and borrowing** — reuse data (e.g. candle history) safely without hidden copies.
- **`Option`** — model missing data and open/flat state explicitly; pair with `if let` / `match`; avoid careless `unwrap()`.
- **State and time** — strategies need memory of the past; signals are **events**, not static inequalities.
- **Engine patterns** — `Option` + `take()` for positions and ids, epsilon for float thresholds, stored `allocation`, slice arguments, and trait-based `run` keep the simulator clear and extensible.

---

## 7. Next topics to explore

- `Result<T, E>` (error handling)
- Lifetimes (advanced borrowing)
- Iterators vs loops
- Performance considerations in Rust
