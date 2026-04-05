# 🦀 Rust Learning Notes

This document captures key Rust concepts I’ve learned while building a trading backtesting engine. The focus is on understanding _why_ Rust works the way it does, not just syntax.

---

# 📦 Option<T> — Handling Absence Safely

## What is Option?

In Rust, values can explicitly represent the possibility of being absent using:

```rust
Option<T>
```

It is defined as:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

---

## Why It Exists

Unlike many languages:

- No `null`
- No undefined references

Instead, Rust forces you to explicitly handle missing values.

---

## Example

```rust
let value: Option<f64> = Some(10.5);
```

or:

```rust
let value: Option<f64> = None;
```

---

## Pattern Matching with `if let`

```rust
if let Some(v) = value {
    println!("Value is {}", v);
}
```

This means:

> “Only run this block if the value exists”

---

## Multiple Values

```rust
if let (Some(a), Some(b)) = (x, y) {
    // both values exist
}
```

Used in my project to ensure both moving averages are available before generating signals.

---

## Why This Matters (Trading Context)

In my trading engine:

```rust
previous_short: Option<f64>
previous_long: Option<f64>
```

These are `None` initially because:

- There isn’t enough historical data yet

Rust forces me to explicitly handle this case before computing signals.

---

## Alternative: match

```rust
match value {
    Some(v) => println!("Value is {}", v),
    None => println!("No value"),
}
```

More explicit but more verbose than `if let`.

---

# ⚠️ unwrap() — Use With Caution

```rust
let v = value.unwrap();
```

This:

- Extracts the value if it exists
- Panics if it is `None`

---

## When to Use

✔ Quick prototypes
✔ Situations where failure is impossible

---

## When NOT to Use

❌ Production code
❌ Anything user-facing
❌ Trading systems (silent crashes = bad)

---

# 🧠 Ownership & Borrowing (Early Notes)

Rust enforces memory safety through ownership.

## Key Idea:

- Each value has one owner
- Data can be borrowed instead of copied

---

## Example

```rust
for candle in &data {
```

This:

- Borrows data
- Prevents moving it
- Allows reuse later

---

## Why It Matters

Without borrowing:

- You lose access to data after iteration
- Causes compile errors

Rust forces explicit data flow.

---

# 🔄 State & Time-Based Logic

A key concept in my project:

> Systems that evolve over time require **state**

Example:

```rust
previous_short: Option<f64>
```

This stores:

- The previous value
- Enables detecting transitions (crossovers)

---

## Insight

Trading strategies are not just about values:

- They are about **changes over time**

Rust models this explicitly through state.

---

# 📈 Signals vs Events

Initial mistake:

```text
If short > long → BUY
```

Correct approach:

```text
If short crosses above long → BUY
```

This required:

- Tracking previous values
- Comparing past vs present

---

# 🧠 Key Takeaways So Far

- Rust forces explicit handling of uncertainty (`Option`)
- State must be modeled explicitly
- Borrowing enables safe data reuse
- Pattern matching is central to control flow

---

# 🎯 Engine patterns (positions, floats, and state)

The sections above introduced `Option` in general (for example, moving-average history). The backtesting **engine** uses `Option` and related ideas in a few extra ways, tied to `src/engine.rs`.

## Using `Option<T>` to model position state

**What it is:** A single variable that is either “no open trade” (`None`) or “in a trade” (`Some(position_data)`). The compiler keeps buy/sell logic honest: you cannot treat a position as open without handling the empty case.

**How it is used in this project:** `position: Option<Position>` starts as `None`. A fill sets `position = Some((entry_price, size, allocation))`. The buy branch checks `position.is_none()` so we only enter when flat; after a sell, the slot is cleared again (via `take()` or by not re-assigning after closing).

**Why it matters:** Flat vs long is exactly the kind of state bugs love. Encoding it as `Option` avoids sentinel values (like `size == 0.0`) and lines up with how we think about the book: either we hold something or we do not.

---

## `Option::take()` — move the value out and reset to `None`

**What it is:** A method that pulls `Some(x)` out of the option, gives you `x`, and leaves the original `Option` as `None` in one step.

**How it is used in this project:** On a sell signal, `position.take()` yields the tuple `(entry_price, size, allocation)` and clears `position` so we are flat without manually assigning `None`. The same idea appears for `open_trade_id.take()` when logging the sell: consume the id that was stored at buy time.

**Why it matters:** Without `take()`, it is easy to read the position, use it, and forget to clear it—or to clear it before you have finished using the data. `take()` makes “extract and empty” the default pattern and reduces duplicate state.

---

## Avoiding floating-point equality

**What it is:** `f64` values are approximations. Comparing with `==` or `!=` is usually wrong for “is this zero?” or “are these equal?” because tiny rounding noise breaks the check.

**How it is used in this project:** Before opening a trade, we only act if there is meaningful cash to allocate, using a small threshold: `allocation > f64::EPSILON` (see the buy branch in `engine::run`). We do not compare prices or sizes with `==` for control flow.

**Why it matters:** Backtests repeat many arithmetic steps. A spurious `== 0.0` can skip a trade or double-count; a threshold-based check stays stable for “do we have enough to size a position?”.

---

## Structuring state to avoid recomputation (stored `allocation`)

**What it is:** Keeping a derived number alongside the inputs when that number is the canonical value for later logic (here, how much cash was actually put into the trade).

**How it is used in this project:** `Position` is `(entry_price, size, allocation)` with `allocation` set at buy time (the cash debited). On exit, PnL uses that stored `allocation`: `pnl = proceeds - allocation` instead of recomputing `size * entry_price`, which could drift slightly from what we debited due to float order.

**Why it matters:** One source of truth for “how much was invested” keeps trade logs, PnL, and cash ledger aligned and easier to audit when exporting CSV or charting results.

---

# 🚀 Next Topics to Explore

- Result<T, E> (error handling)
- Traits (strategy abstraction)
- Lifetimes (advanced borrowing)
- Iterators vs loops
- Performance considerations in Rust

---
