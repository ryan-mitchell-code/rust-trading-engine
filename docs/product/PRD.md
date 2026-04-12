# Trading Strategy Backtesting & Visualization System (PRD)

## 1. Objective

Build an interactive system to:

> **backtest, compare, and analyze trading strategies visually using historical market data**

The system should prioritize:

* clarity of results
* understanding of risk vs return trade-offs
* fast iteration and experimentation

---

## 2. Current State

### Backend (Rust)

* Backtesting engine:

  * Position management
  * Trade lifecycle (BUY → SELL)

* Strategy abstraction via `Strategy` trait

* Implemented strategies:

  * Moving Average crossover
  * RSI mean-reversion
  * Random
  * Buy & Hold (benchmark)

* Metrics:

  * Return %
  * Sharpe ratio
  * Max drawdown
  * Drawdown duration
  * Win rate, avg PnL

* Data:

  * Binance OHLC integration

* Output:

  * `BacktestRun { market, results }`

* Configuration:

  * `RunConfig` bundles MA windows and RSI parameters for each arena run
  * `POST /run` accepts MA and RSI fields (see §6) with defaults

---

### Frontend (React + TypeScript)

* Charts:

  * Market OHLC (candlestick)
  * Equity curves
  * Drawdown curves
  * Candlestick time-range presets (e.g. All, 1M, 3M, 6M, 1Y) via chart time scale

* Interaction:

  * Run backtest via API
  * Dataset selection (BTC, ETH)
  * Strategy selection (cards + comparison table)
  * View toggle (charts / table)
  * Collapsible **Settings** panel for strategy parameters (MA short/long, RSI period and thresholds)

* Visual features:

  * Strategy summary cards
  * Best strategy highlighting
  * Trade markers on the candlestick chart when a strategy is selected

---

## 3. Core Capabilities

### 3.1 Data

* Load historical OHLC data
* Support multiple datasets
* (Future) support multiple timeframes

---

### 3.2 Simulation

* Execute strategies over shared dataset
* Track:

  * capital
  * positions
  * trades
  * equity curve

---

### 3.3 Evaluation

* Compute performance metrics
* Compare strategies vs benchmark (Buy & Hold)
* Rank strategies using scoring system
* Per-strategy **relative return vs Buy & Hold** in results (`summary.relative_return`)
* (Future) extend relative / cross-strategy metrics in backend as needed

---

### 3.4 Visualization

* Provide:

  * market context (OHLC candlestick chart)
  * performance (equity)
  * risk (drawdown)

* Enable:

  * multi-strategy comparison
  * strategy highlighting
  * trade inspection

---

### 3.5 Interaction

* Trigger backtests from UI
* Select dataset
* Select and highlight strategies
* View charts or table
* Inspect trades visually

---

## 4. Architecture Principles

* Backend is the **source of truth**
* UI is primarily **visualization + interaction layer**
* Derived metrics may temporarily exist in UI but should migrate to backend
* Keep API contracts explicit and simple
* Prefer clarity over abstraction

---

## 5. Roadmap

---

### Phase 1 — Visualization ✅ Complete

Goal:

> Make results understandable

* [x] Market price chart
* [x] Equity chart
* [x] Drawdown chart
* [x] Strategy comparison table
* [x] Buy & Hold benchmark
* [x] Trade markers

---

### Phase 2 — Interaction 🚧 In Progress

Goal:

> Make system exploratory

#### Completed

* [x] Run backtest from UI (`POST /run`)
* [x] Dataset selection
* [x] Strategy selection & highlighting
* [x] View toggle (charts / table)
* [x] Strategy summary cards
* [x] Parameter controls for **Moving Average** (short / long) and **RSI** (period, overbought, oversold), sent with `POST /run`
* [x] Collapsible settings panel + inline param summary when collapsed

#### Remaining

* [ ] Strategy enable/disable (API-driven — which strategies run in the arena)
* [ ] Timeframe selection (1d, 4h, etc.) end-to-end (UI + API + data load)

---

### Phase 3 — Strategy Research

Goal:

> Enable experimentation

* [ ] Parameter sweeps
* [ ] Compare multiple runs
* [ ] Deeper relative performance vs benchmark (beyond current `relative_return` + scoring)
* [ ] Alternative scoring models

---

### Phase 4 — Portfolio Simulation

Goal:

> Combine strategies

* [ ] Allocate capital across strategies
* [ ] Portfolio metrics
* [ ] Compare portfolio vs individual strategies

---

### Phase 5 — Advanced

* [ ] Multi-dataset comparison
* [ ] Strategy optimization
* [ ] Advanced visualizations:

  * risk vs return scatter
  * drawdown duration charts

---

## 6. API Design

### Endpoint

`POST /run`

---

### Request (Current)

```json
{
  "dataset": "BTCUSDT",
  "interval": "1d",
  "ma_short": 10,
  "ma_long": 50,
  "rsi_period": 14,
  "rsi_overbought": 70,
  "rsi_oversold": 30
}
```

`ma_*` and `rsi_*` fields are optional; defaults match the UI (e.g. MA 10/50, RSI 14/70/30).

---

### Request (Future)

Structured per-strategy enablement and configuration (replacing the flat MA/RSI fields above):

```json
{
  "dataset": "BTCUSDT",
  "interval": "1d",
  "strategies": [
    { "type": "moving_average", "short": 10, "long": 50 },
    { "type": "buy_and_hold" }
  ]
}
```

---

### Response

`BacktestRun` (JSON). Shape matches Rust `engine::BacktestRun` / `serde` export.

**`market`** — shared series, one row per bar (aligned with strategy curves):

```json
[
  ["2024-01-01T00:00:00+00:00", 42000.0, 43000.0, 41500.0, 42500.0],
  ...
]
```

Each row is **`[timestamp, open, high, low, close]`** (numbers are `f64`).

**`results`** — array of per-strategy objects: `name`, `summary`, `equity_curve`, `drawdown_curve`, `trades`, etc.

---

### Notes

* API is intentionally minimal
* MA/RSI parameters are supported; **arena strategies are still fixed** (MA, RSI, random, buy-and-hold) until enable/disable lands
* Future expansion:

  * strategy enable/disable and richer `strategies` JSON (see above)
  * additional strategy types and parameters

---

## 7. Immediate Next Steps

1. **Timeframe selection** — user-selectable interval in UI and `POST /run`, backed by Binance load
2. **Strategy enable/disable** — API + arena to run a subset of strategies
3. **Structured strategy config** — optional `strategies` array (or equivalent) on `POST /run`
4. **Phase 3** — sweeps, multi-run comparison, scoring experiments

---

## 8. Key Insights

* Performance must be evaluated relative to a benchmark
* Drawdown is as important as return
* Visualization should answer questions, not just display data
* Simplicity enables faster iteration

---

## 9. Success Criteria

A user can:

* select dataset
* configure strategies
* run backtest
* understand results visually

The system clearly communicates:

* performance
* risk
* trade-offs
