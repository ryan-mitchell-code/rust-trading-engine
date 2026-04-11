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

  - Equity curve (`outputs/equity_*.csv`)
  - Trade log (`outputs/trades_*.csv`)

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

## Day 3 — Metrics, Risk, and Strategy Evaluation

### What I built

- Introduced a dedicated metrics system:

  - Total trades
  - Win rate
  - Total and average PnL

- Added risk-focused metrics:

  - Max drawdown (%)
  - Drawdown duration (DD bars)

- Implemented Sharpe ratio using equity curve returns
- Built a multi-strategy comparison system (“arena”)
- Added Buy & Hold as a baseline benchmark
- Introduced score-based ranking combining:

  - Return
  - Drawdown
  - Sharpe ratio

- Added score breakdown for transparency
- Normalized metrics to ensure fair comparison
- Refactored architecture:

  - Engine produces raw metrics only
  - Evaluation logic moved to main

---

### What I learned (Rust)

- Separating computation from evaluation leads to cleaner architecture
- Returning structured data is more powerful than printing directly
- Keeping logic explicit improves debuggability and iteration speed
- Small refactors (moving responsibilities) have large impact on clarity

---

### What I learned (Trading)

- Profit alone is a poor measure of strategy quality
- Drawdown is a critical risk metric, not just a side statistic
- A strategy that “loses less” can be better than one that “wins more”
- Sharpe ratio highlights consistency, not just outcome
- Benchmarking against Buy & Hold is essential
- Random strategies can outperform naive strategies in weak markets

---

### What I observed

- My moving average strategy performed worse than both random and buy & hold
- Random strategies sometimes outperform due to reduced exposure
- Buy & Hold can remain in drawdown for the entire dataset in weak markets
- Strategy evaluation is highly sensitive to how metrics are weighted

---

### What I got wrong

- Initially treated metrics as outputs rather than inputs to decision-making
- Assumed Sharpe ratio would be the primary driver of performance
- Underestimated the importance of normalization when combining metrics
- Mixed evaluation logic into the engine before refactoring

---

### Key insights

A trading system is not just about generating trades.

It is about:

> **evaluating strategies correctly across return, risk, and consistency**

and understanding the trade-offs between them.

---

### Next questions

- How sensitive is ranking to scoring weights?
- What makes a “good” strategy under different market conditions?
- Can I design multiple scoring models and compare them?
- How should I visualize these results effectively?

---

### Next steps

- Refactor engine to return structured data (prepare for UI)
- Experiment with different scoring models
- Improve strategy quality (add new strategies)
- Introduce a visualization layer (React)
