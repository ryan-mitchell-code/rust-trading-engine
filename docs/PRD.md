# Trading Strategy Backtesting & Visualization System (PRD)

## 1. Objective

Build an interactive system to:

> **backtest, compare, and analyze trading strategies visually using historical market data**

The system should prioritize:

* clarity of results
* understanding of trade-offs (return vs risk)
* fast iteration and experimentation

---

## 2. Current State

### Backend (Rust)

* Backtesting engine with:

  * Position management
  * Trade execution (BUY → SELL lifecycle)
* Strategy abstraction via `Strategy` trait
* Implemented strategies:

  * Moving Average crossover
  * Random
  * Buy & Hold (benchmark)
* Metrics:

  * Return %
  * Sharpe ratio (per-period)
  * Max drawdown
  * Drawdown duration
  * Win rate, avg PnL
* Scoring system:

  * Normalized components (return, drawdown, sharpe)
* Data:

  * Binance OHLC integration
* Output:

  * `BacktestRun { market, results }`

---

### Frontend (React + TypeScript)

* Strategy comparison table
* Charts (Recharts):

  * Market price
  * Equity curves (multi-strategy)
  * Drawdown curves
* Visual features:

  * Best strategy highlighting
  * Buy & Hold as benchmark

---

## 3. Core Capabilities

### 3.1 Data

* Load historical market data (OHLC)
* Support multiple datasets (future)

### 3.2 Simulation

* Execute strategies over shared dataset
* Track:

  * capital
  * positions
  * trades
  * equity over time

### 3.3 Evaluation

* Compute performance metrics
* Compare strategies against benchmark (Buy & Hold)
* Rank strategies via scoring system

### 3.4 Visualization

* Provide:

  * market context (price chart)
  * performance (equity)
  * risk (drawdown)
* Enable visual comparison across strategies

### 3.5 Execution (Planned)

* Trigger backtests dynamically from UI
* Pass configuration to backend
* Return results in real-time

---

## 4. Architecture Principles

* Backend is the **source of truth**
* UI is **read-only visualization + control layer**
* No metric computation in the frontend
* Keep data contracts simple and explicit
* Prefer clarity over abstraction

---

## 5. Roadmap

---

### Phase 1 — Visualization (Complete)

Goal:

> Make results understandable

#### Features

* [x] Market price chart
* [x] Equity chart (multi-strategy)
* [x] Drawdown chart
* [x] Strategy comparison table
* [x] Buy & Hold benchmark
* [x] Best strategy highlighting

---

### Phase 2 — Interaction (Next)

Goal:

> Make the UI exploratory and interactive

#### Features

##### 2.1 Run Backtest from UI

* Trigger simulation via API
* Return `BacktestRun`

##### 2.2 Dataset Selection

* Select market (e.g. BTC, ETH)
* Select timeframe / interval

##### 2.3 Strategy Selection

* Enable/disable strategies before execution

##### 2.4 Parameter Controls

* Adjust strategy parameters (e.g. MA windows)

##### 2.5 Trade Markers (High Priority)

* Display BUY/SELL points on price chart

##### 2.6 Strategy Highlight & Toggle

* Click to focus on strategy
* Show/hide strategies in charts

---

### Phase 3 — Strategy Research

Goal:

> Improve and evaluate strategies more deeply

#### Features

* Parameter sweeps / multiple runs
* Relative performance vs benchmark
* Strategy comparison views
* Experiment with scoring models

---

### Phase 4 — Portfolio Simulation

Goal:

> Combine multiple strategies

#### Features

* Allocate capital across strategies
* Portfolio-level metrics
* Compare portfolio vs individual strategies

---

### Phase 5 — Advanced (Optional)

* Multi-dataset comparison
* Strategy evolution / optimization
* Advanced visualizations:

  * risk vs return scatter plots
  * drawdown duration analysis

---

## 6. API Design (Planned)

### Endpoint

`POST /run`

---

### Request

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

```json
BacktestRun
```

---

### Design Notes

* UI defines *what to run*
* Backend defines *how it runs*
* Keep API minimal initially
* Extend incrementally

---

## 7. Immediate Tasks

### Task 1 — Trade Markers

* Extract trades from results
* Map to timestamps
* Render on price chart

---

### Task 2 — Strategy Toggle

* Add selection state in UI
* Filter chart rendering

---

### Task 3 — Strategy Highlight

* Click table row → highlight chart line

---

### Task 4 — API Integration (Backend)

* Add `/run` endpoint
* Return `BacktestRun`
* Replace static JSON in UI

---

## 8. Key Insights

* A strategy must be evaluated relative to a benchmark
* Buy & Hold is a proxy for market performance
* Equity shows outcome; drawdown shows risk
* Visualization must answer specific questions, not just display data

---

## 9. Future Questions

* How do strategies behave across different markets?
* What defines a “good” strategy under different conditions?
* How should risk vs return be visualized effectively?
* Can strategies be combined for better outcomes?

---

## 10. Success Criteria

The system is successful when:

* A user can:

  * select a dataset
  * configure strategies
  * run a backtest
  * understand results visually

* The system clearly communicates:

  * performance
  * risk
  * trade-offs between strategies

---
