# Project Context

This is a Rust-based trading backtesting engine built as a learning project.

The long-term goal is to evolve this into a **strategy simulation platform** where multiple trading algorithms ("bots") can be compared and evaluated visually.

---

## 🎯 Goals

- Learn Rust through a real-world system
- Understand trading strategy design and risk
- Build a modular and extensible simulation engine
- Enable comparison of multiple trading strategies

---

## 📊 Simulation Goal

The system should support:

- Running trading strategies on historical data
- Tracking capital over time
- Producing outputs suitable for visualisation
- Comparing performance across strategies

A “bot” will eventually represent:

- A strategy
- Capital
- Internal state

---

## 📈 Visualisation Goal

Visual feedback is essential for learning.

The system produces:

- Equity curve (capital over time) and per-bar drawdown series (from the engine)
- Trade log (buy/sell events and outcomes)
- Shared market series in `results.json` (per bar: timestamp + OHLC)

These outputs are available as:

- **JSON** for the React UI (`outputs/results.json`)
- **CSV** for Excel / Google Sheets

---

## 🤖 AI-Assisted Development

Cursor is used as a development assistant.

Guidelines:

- The developer (human) defines architecture and intent
- Cursor assists with implementation and explanations
- Code must remain understandable and explainable
- Avoid over-engineered or overly complex solutions

---

## 📚 Documentation Goals

This project documents both implementation and learning.

### Objectives

- Capture key Rust concepts as they are introduced
- Explain trading concepts in simple terms
- Maintain a glossary of domain-specific terminology
- Show progression from simple to more advanced systems

---

### Approach

- Keep documentation concise and practical
- Tie explanations directly to code usage
- Prefer clarity over completeness
- Avoid duplicating information unnecessarily

---

### Files

- `/docs/rust-learning.md` → Rust concepts and learnings
- `/docs/dev-log.md` → Progress, decisions, mistakes
- `/docs/glossary.md` → Trading concepts
- `/docs/ui-rules.md` → UI conventions (read-only, types mirror Rust)
- `README.md` → High-level overview
- `ui/README.md` → How to run the front-end

---

## ⚙️ Current Features

- **Data**: Binance klines (with local JSON cache under `outputs/`), or CSV via `data::load_csv` for experiments
- **Strategies**: moving average crossover, random, buy & hold
- **Engine**: positions, capital, equity curve, per-bar drawdown series, trade log
- **Metrics**: returns, max drawdown, drawdown duration, per-period Sharpe, scoring vs buy & hold
- **Export**: `BacktestRun` JSON (`market` + `results`), per-strategy CSVs, CLI comparison table
- **UI**: React app — strategy table, market candlestick chart, equity, and drawdown charts

---

## 🧱 Design Principles

- Keep code simple and readable
- Prefer explicit state over hidden logic
- Avoid premature optimisation
- Build incrementally with clear feedback loops

---

## ⚠️ Constraints

- No external *trading* libraries (networking for market data is OK)
- Focus on learning, not profitability
- Code should be idiomatic Rust where possible
- Avoid unnecessary abstraction early

---

## 🚀 Next Steps (Ideas)

- Tune strategies and scoring
- CLI or config for symbol / interval / limits
- More metrics or reporting as needed
