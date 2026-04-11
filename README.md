# 🚀 Rust Trading Engine (Learning Project)

A Rust-based trading backtesting and strategy evaluation engine built as a hands-on learning project.

This project focuses on building a **modular, extensible system** for testing and comparing trading strategies, while learning Rust and core quantitative trading concepts.

---

## 🎯 Goals

- Learn Rust through building a real-world system
- Understand trading strategy design and evaluation
- Explore systematic decision-making (not just profitability)
- Experiment with AI-assisted development using Cursor

---

## ⚙️ Current Capabilities

### 🧱 Backtesting Engine

- Executes BUY / SELL lifecycle
- Tracks capital, positions, and PnL
- Supports pluggable strategies via trait system
- Per-bar **equity** and **drawdown vs running peak** (computed in Rust; not recomputed in the UI)

### 📊 Metrics & Analysis

- Return (%)
- Max drawdown (%)
- Drawdown duration (DD bars)
- **Per-period** Sharpe ratio from the equity curve (mean / sample std dev of step returns; **not** annualized)

### 🧠 Strategy Evaluation

- Multi-strategy comparison (“arena”)
- Relative performance vs Buy & Hold
- Score-based ranking using:

  - return
  - drawdown
  - Sharpe ratio

- Normalized scoring for fair comparison
- Score breakdown (transparency of decision)

### 📈 Strategies Implemented

- Moving Average Crossover
- Random (baseline)
- Buy & Hold (benchmark)

### 📥 Data

- Default: **Binance** spot klines (`BTCUSDT`, `1d`, limit 1000) with a **local JSON cache** under `outputs/` to avoid repeat API calls
- Optional: CSV loading remains available in code for ad-hoc use (`data::load_csv`)

### 📤 Outputs

- **`outputs/results.json`**: structured **`BacktestRun`** — shared **market** (timestamp + close per bar) plus **per-strategy** results (summaries, equity as `f64[]`, drawdown as `f64[]`, trades)
- **CSV**: equity curve and trades per strategy (for spreadsheets)
- **CLI**: comparison table with full metrics

### 🖥️ UI (React)

- Reads `results.json` (Vite dev server serves workspace `outputs/results.json`)
- Strategy comparison table, **market price**, **equity**, and **drawdown** charts (Buy & Hold shown as a neutral benchmark style)

---

## 🏗️ Architecture Overview

```text
Market data (Binance API + cache, or CSV)
   ↓
Strategies
   ↓
Engine (execution + state)
   ↓
Metrics (performance measurement)
   ↓
Evaluation (scoring + ranking)
   ↓
Output: CLI + CSV + results.json
   ↓
UI (read-only visualization)
```

Key design principles:

- Separation of concerns (engine vs evaluation)
- Explicit state over hidden logic
- Deterministic, testable components
- Incremental, iterative development

---

## 🧠 Key Concepts Learned

### Rust

- Ownership & borrowing (`Option<T>`, `take()`)
- Traits and generics (`Strategy` abstraction)
- Modular system design
- Avoiding unnecessary cloning

### Trading

- Profit ≠ good strategy
- Importance of risk (drawdown)
- Consistency vs volatility (Sharpe ratio)
- Benchmarking against Buy & Hold
- Backtesting as a decision tool, not prediction

---

## 📈 Roadmap

### 🔜 Next Steps

- Experiment with multiple scoring models
- Improve strategy quality and parameters
- Optional: CLI flags for symbol / interval / data source (without editing `main.rs`)

### 🚀 Future Goals

- Strategy parameter tuning and sweeps
- Parallel backtesting
- Paper trading via exchange APIs
- More advanced metrics (e.g. volatility, drawdown duration as % of window)

---

## 📓 Dev Log

See `/docs/dev-log.md` for reflections, mistakes, and learning notes throughout development.

---

## 💡 Project Philosophy

This project is not about finding a profitable strategy.

It’s about:

> **building the tools to evaluate strategies correctly**

and developing a deep understanding of both Rust and systematic trading.
