# Dev Log

## Day 1 — Initial Backtester

Built a basic moving average crossover system.

### What I learned

- Rust module structure
- File parsing
- Basic strategy implementation

### What I got wrong

- No concept of fees or slippage
- Signals ≠ actual trades

### Next steps

- Add trade execution logic

---

## Day 1 continued — Trade Execution & Capital Tracking

Extended the system to simulate actual trades instead of just emitting signals.

### What I built

- Added position tracking using `Option<Position>`
- Implemented trade execution (BUY → SELL lifecycle)
- Introduced capital tracking (starting at 10,000)
- Added position sizing (10% of capital per trade)
- Implemented trade logging with trade IDs
- Exported:

  - Equity curve (`logs/equity.csv`)
  - Trade log (`logs/trades.csv`)

---

### What I learned (Rust)

- Using `Option<T>` to model state is cleaner than relying on numeric checks
- `Option::take()` is a powerful way to safely move and clear state
- Structuring data (e.g. storing allocation in position) avoids recomputation and simplifies logic
- Simpler code is often better than abstract or “clever” code

---

### What I learned (Trading)

- A strategy producing signals does not mean it is profitable
- Moving average crossovers are lagging indicators
- The system often enters trades after the move has already started
- Losses were frequent and gains were smaller than losses
- Backtesting is essential to validate assumptions

---

### What I observed

- The strategy consistently lost money over time (~1.5% drawdown)
- Most trades resulted in small losses
- Occasional wins were not large enough to offset losses
- Equity curve showed a gradual downward trend

---

### What I got wrong

- Initially assumed a “working” strategy would show some profitability
- Underestimated how noisy markets are
- Focused on signals instead of outcomes (PnL and capital)

---

### Key insight

A correct implementation does not imply a good strategy.

---

### Next questions

- How can I reduce false signals?
- Would a simpler strategy perform better?
- What happens if I compare multiple strategies on the same data?

---

### Next steps

- Introduce a second strategy for comparison
- Build toward running multiple strategies (“bots”) on the same dataset
- Compare performance using equity curves

## Day 2 — Strategy Abstraction & Engine Refactor

### What I built

- Introduced a `Strategy` trait
- Refactored engine to support multiple strategies
- Added a random strategy for comparison
- Removed data cloning by passing references
- Extracted helper functions for clarity

---

### What I learned (Rust)

- Generics (`<S: Strategy>`) allow flexible and reusable designs
- Passing by reference avoids unnecessary memory usage
- Small helper functions improve readability without overengineering

---

### What I learned (System Design)

- Separating strategy from execution is critical
- Designing for extension early simplifies future changes
- Clean data flow makes the system easier to reason about

---

### What I observed

- The same engine can now run different strategies
- Outputs are directly comparable across strategies
- The system is evolving into a reusable simulation framework

---

### Next steps

- Add unit tests for core logic
- Compare multiple strategies visually
- Introduce simple performance metrics
