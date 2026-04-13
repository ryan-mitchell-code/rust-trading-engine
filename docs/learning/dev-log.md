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

## Day 4 — Visualization, Market Context, and Data Evolution

### What I built

* Introduced a React + TypeScript UI layer
* Implemented a strategy comparison table:

  * Return %
  * Drawdown %
  * Sharpe ratio
  * Score
* Built an Equity Chart using Recharts:

  * Multiple strategies plotted together
  * Highlighted best-performing strategy
* Added a Drawdown Chart:

  * Visualizes depth and duration of losses
  * Uses dynamic scaling for readability
* Integrated real market data using Binance OHLC API
* Refactored backend output:

  * Introduced `BacktestRun`

    * `market` (shared price data)
    * `results` (per-strategy outputs)
* Added a Market Price chart to provide context

---

### What I learned (Frontend / UI)

* UI should be a **pure visualization layer**, not a computation layer
* Clean data contracts between backend and frontend remove complexity
* Charting libraries (Recharts) are far better than building custom rendering
* Small visual decisions (scaling, opacity, line weight) have a big impact on readability
* Good UI is about **clarity of insight**, not just displaying data

---

### What I learned (System Design)

* Separating **market data from strategy results** is a better model than duplicating data per strategy
* Designing data structures for consumption (UI) simplifies the entire system
* Avoiding transformation layers in the UI keeps architecture clean
* A small refactor (BacktestRun) significantly improved extensibility

---

### What I learned (Trading)

* Buy & Hold is effectively a **proxy for the market**
* A strategy should always be evaluated relative to a benchmark
* Drawdown provides a very different perspective than equity:

  * Equity shows outcome
  * Drawdown shows pain
* Strategies that look similar on equity can differ significantly in risk
* Market conditions (trend vs flat) heavily influence strategy performance

---

### What I observed

* My initial dataset was too flat, hiding meaningful differences between strategies
* Switching to real OHLC data immediately made:

  * equity curves more dynamic
  * drawdowns more visible
* Drawdown charts can appear “flat” if scaling is incorrect
* Buy & Hold overlaps heavily with market data, but is still essential as a benchmark

---

### What I got wrong

* Initially tried to normalize/transform data in the UI instead of fixing backend structure
* Underestimated the importance of **chart scaling**
* Assumed drawdown would always be visually distinct without adjusting the axis
* Considered removing Buy & Hold due to visual redundancy, rather than reframing it as a benchmark

---

### Key insights

Visualization is not just about displaying data.

It is about:

> **providing context that allows meaningful comparison and decision-making**

and ensuring that:

> **every chart answers a specific question**

---

### Next questions

* How can I visualize **trade decisions** (entries/exits) on the market?
* How do strategies behave under different datasets (BTC vs equities)?
* Can I compare strategies relative to the benchmark more explicitly?
* What is the best way to represent **risk vs return trade-offs visually**?

---

### Next steps

* Add trade markers (buy/sell) to the price chart
* Introduce dataset switching (multiple markets)
* Improve comparison vs Buy & Hold (relative performance view)
* Explore portfolio-level simulation (multiple strategies combined)

## Day 5 — PRD accuracy, engine hygiene, and trading handbook

### What I built

* **PRD** (`docs/product/PRD.md`): realigned with the codebase—current execution model (same-bar close), fixed arena strategy set, metrics (including composite score and drawdown series), API vs UI gaps (e.g. interval hardcoded in UI), and **Phase 2.5** framed as next-bar execution, fees, slippage, and a single fill path with tests.
* **Backend refactor** (BacktestParams, OpenPosition, `equity_curve` module—see git log on `main`):

  * `BacktestParams` for initial capital and position fraction (shared across arena runs; room for future costs).
  * `OpenPosition` struct instead of a tuple for entry, size, and allocation.
  * New `equity_curve` module: Sharpe and per-bar drawdown **series** as pure functions on a finished curve; `metrics.rs` stays the **incremental** bar-by-bar accumulator—documented so responsibilities stay clear.
* **Trading handbook** (`docs/reference/trading-handbook.md`): from-scratch narrative (markets → bars → spot/PnL → strategies → honest backtesting → metrics → repo map) plus **section 8** quick reference. Renamed from “glossary” so the name matches the learning goal; `glossary.md` remains a short redirect for old links.
* **Discoverability**: root `README.md`, `docs/README.md`, and `docs/project/context.md` point to the handbook.

---

### What I learned (documentation)

* A **single accurate PRD** saves time when onboarding future-you; calling out lookahead and UI vs API truth avoids false assumptions.
* Splitting **“live metrics during the run”** from **“analytics on the equity vector”** matches how people reason about backtests and keeps modules testable.

---

### What I learned (Rust / design)

* Named structs for position state reduce tuple-index mistakes before execution logic gets richer (pending orders, next-bar fills).
* Centralizing run parameters in one type makes the next features (fees, slippage) a smaller conceptual jump.

---

### Next questions

* How much does next-bar open change rankings versus same-bar close on our current datasets?
* What fee/slippage defaults best match Binance spot for documentation examples?

---

### Next steps

* Implement Phase 2.5 execution (next-bar open, fees, slippage) with regression tests on small candle fixtures.
* Wire **interval** (and later strategy toggles) through the UI to match `POST /run`.
* Keep extending the trading handbook as execution and portfolio features land.
