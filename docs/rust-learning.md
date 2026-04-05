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

# 🚀 Next Topics to Explore

- Result<T, E> (error handling)
- Traits (strategy abstraction)
- Lifetimes (advanced borrowing)
- Iterators vs loops
- Performance considerations in Rust

---
