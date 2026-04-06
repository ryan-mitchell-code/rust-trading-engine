# Coding Rules

- Prefer simple, readable Rust over clever code
- Avoid unnecessary cloning
- Use Option and Result properly (no excessive unwrap)
- Keep functions small and focused
- Use structs to model state clearly
- Avoid deeply nested logic

## Data Flow & Ownership

- Prefer passing data by reference (`&[T]`) instead of cloning
- Avoid unnecessary allocations and copies
- Functions should not take ownership unless required

---

## Engine Design

- The engine should be generic over strategies (`Strategy` trait)
- Strategies must not depend on engine internals
- The engine handles:

  - capital
  - execution
  - logging

---

## Logging & Output

- All outputs should be structured for analysis (CSV-first design)
- Avoid printing values that are not also captured in logs
- Keep output formats consistent across strategies

---

## Code Structure

- Extract small helper functions when they reduce duplication
- Avoid repeating formatting or calculation logic
- Prefer clarity over abstraction

---

## Testing Philosophy

- Core logic should be testable in isolation
- Avoid tightly coupling logic to I/O (e.g. CSV writing)
- Prefer deterministic behaviour in tests (avoid randomness)

## State & Simplicity Rules

- Prefer using `Option<T>` to represent presence/absence of state
- Do NOT combine `Option` checks with floating point checks

  - If `position: Option<T>` is used:

    - `None` = no position
    - `Some(_)` = position exists

  - Avoid redundant checks like `size > f64::EPSILON` when using `Option`

---

## Floating Point Safety

- Do NOT compare floating point values using `==` or `!=`
- Use `f64::EPSILON` only when comparing numerical thresholds
- Avoid unnecessary floating point checks when state can be represented structurally (e.g. with `Option`)

---

## Trade Lifecycle Rules

- A trade starts on BUY and ends on SELL
- Each trade must have a unique `trade_id`
- `trade_id` should be assigned at BUY time
- BUY and SELL must share the same `trade_id`

---

## Trade Data Consistency

- Use consistent definitions:

  - `allocation` = capital committed at BUY
  - `proceeds` = value received at SELL
  - `pnl` = proceeds - allocation

- Do NOT recompute values that can be stored (e.g. allocation)

- Prefer storing values in state over recalculating later

---

## Code Simplicity

- Prefer simple and readable code over abstraction
- Avoid unnecessary helper functions unless they clearly improve clarity
- Avoid chaining complex iterator logic when a simple expression is clearer

Example:

- Prefer:
  `position.map(|(_, size, _)| size).unwrap_or(0.0)`

- Avoid:
  unnecessary `.filter(...)` when state already guarantees validity

## Testing Rules

- All core logic must have unit tests
- Add tests when introducing new functionality
- Prefer testing behaviour over implementation details

### Strategy Testing

- Strategies should be tested independently of the engine
- Tests should verify signal behaviour, not profitability
- Avoid relying on randomness in tests (use deterministic inputs where possible)

### Engine Testing

- Test small components (e.g. capital calculation, pnl)
- Avoid testing full engine loops initially

### General

- Tests should be:

  - simple
  - deterministic
  - fast

- Do not couple tests to file I/O or CSV output

## Documentation Rules

When introducing new Rust concepts:

- Suggest updates to /docs/rust-learning.md
- Do NOT automatically overwrite content
- Keep explanations simple and tied to actual usage
