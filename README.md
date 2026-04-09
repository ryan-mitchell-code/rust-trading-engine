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

### 📊 Metrics & Analysis

- Return (%)
- Max drawdown (%)
- Drawdown duration (DD bars)
- Sharpe ratio (risk-adjusted return)

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

### 📤 Outputs

- CSV export (equity curve + trades)
- CLI comparison table with full metrics

---

## 🏗️ Architecture Overview

```text
Strategies
   ↓
Engine (execution + state)
   ↓
Metrics (performance measurement)
   ↓
Evaluation (scoring + ranking)
   ↓
Output (CLI / CSV)
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

- Separate engine from I/O (prepare for UI)
- Return structured results (equity + trades)
- Experiment with multiple scoring models
- Improve strategy quality

### 🚀 Future Goals

- React UI for visualization (charts, comparisons)
- Strategy parameter tuning
- Parallel backtesting
- Paper trading via exchange APIs
- More advanced metrics (volatility, drawdown duration %)

---

## 📓 Dev Log

See `/docs/dev-log.md` for reflections, mistakes, and learning notes throughout development.

---

## 💡 Project Philosophy

This project is not about finding a profitable strategy.

It’s about:

> **building the tools to evaluate strategies correctly**

and developing a deep understanding of both Rust and systematic trading.
