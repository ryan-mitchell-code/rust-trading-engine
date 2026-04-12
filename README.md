# 🚀 Rust Trading Engine (Learning Project)

A Rust-based trading backtesting and strategy evaluation engine built as a hands-on learning project.

This project focuses on building a **modular, extensible system** for testing and comparing trading strategies, while learning Rust and core quantitative trading concepts.

**Documentation:** [docs/README.md](docs/README.md) — product spec, learning notes, glossary, UI rules, and dev log (organized by folder).

---

## ▶️ Run locally (API + UI — default)

**Prerequisites:** [Rust](https://rustup.rs/) (stable), **Node 18+** and npm.

From the repository root, start the **HTTP API** and the **Vite** dev server in one go:

```bash
./scripts/dev.sh
```

Or:

```bash
npm run dev
```

This:

1. Runs the backend with **`--serve`** — Axum listens on **`http://127.0.0.1:3000`**, exposing **`POST /run`** (dataset + interval → `BacktestRun` JSON).
2. Starts the React app; Vite proxies **`/run`** to that API (see `ui/vite.config.ts`).

Open **`http://localhost:5173`**, pick a dataset, and click **Run Backtest**. The first Rust compile can take about a minute; the script waits until port **3000** is open before starting the UI.

**Manual (two terminals):**

```bash
cargo run --manifest-path backend/Cargo.toml -- --serve   # terminal 1 — API
npm --prefix ui install && npm --prefix ui run dev       # terminal 2 — UI
```

**One-shot CLI backtest** (no API: runs once, prints the comparison table, writes `outputs/results.json` and CSVs):

```bash
cargo run --manifest-path backend/Cargo.toml
```

Verbose engine logs: add `-v` or `--verbose` to that command (not used by `--serve`).

**UI only** (Vite without the API — `Run Backtest` will fail until something serves `POST /run`):

```bash
npm --prefix ui install && npm --prefix ui run dev
```

The Vite dev server can still expose **`/results.json`** from `outputs/results.json` for optional static viewing (`ui/backtest-results-plugin.ts`); the dashboard flow is **API-first** via **`POST /run`**.

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

- **`outputs/results.json`**: structured **`BacktestRun`** — shared **market** (timestamp + OHLC per bar) plus **per-strategy** results (summaries, equity as `f64[]`, drawdown as `f64[]`, trades)
- **CSV**: equity curve and trades per strategy (for spreadsheets)
- **CLI**: comparison table with full metrics

### 🖥️ UI (React)

- **Default:** **`POST /run`** to the Rust API (same `BacktestRun` JSON as file export)
- **Optional dev:** Vite can serve workspace **`outputs/results.json`** at `/results.json` for static viewing
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
UI (POST /run to API, or optional static results.json in dev)
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

See [`docs/learning/dev-log.md`](docs/learning/dev-log.md) for reflections, mistakes, and learning notes throughout development. The full doc index is [`docs/README.md`](docs/README.md).

---

## 💡 Project Philosophy

This project is not about finding a profitable strategy.

It’s about:

> **building the tools to evaluate strategies correctly**

and developing a deep understanding of both Rust and systematic trading.
