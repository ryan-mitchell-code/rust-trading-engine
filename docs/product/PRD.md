# Trading Strategy Backtesting & Visualization System (PRD)

---

## 1. Objective

Build an interactive system to:

> **backtest, compare, and analyze trading strategies visually using historical market data**

The system should prioritize:

* clarity of results
* understanding of **risk vs return trade-offs**
* fast iteration and experimentation
* increasing realism over time

---

## 2. Current State

---

### Backend (Rust)

#### Engine

* Backtesting engine with:

  * Position management
  * Trade lifecycle (BUY → SELL)
  * Capital tracking
* Execution model:

  * Currently **same-candle execution (to be improved)**

---

#### Strategies

* Moving Average crossover (trend-following)
* RSI mean-reversion
* Random (baseline)
* Buy & Hold (benchmark)

---

#### Metrics

* Return %
* Sharpe ratio
* Max drawdown
* Drawdown duration
* Win rate
* Avg PnL
* Relative return vs Buy & Hold

---

#### Data

* Binance OHLC (open, high, low, close)
* Cached locally

---

#### API

* `POST /run`
* Accepts:

  * dataset
  * interval
  * MA parameters
  * RSI parameters
* Uses `RunConfig` internally

---

---

### Frontend (React + TypeScript)

---

#### Visualization

* Candlestick chart (OHLC)
* Equity curves (multi-strategy)
* Drawdown chart
* Trade markers on chart

---

#### Interaction

* Run backtest via API
* Dataset selection
* Strategy selection (cards + table)
* Chart/table toggle
* Time range controls (via `setVisibleRange`)
* Collapsible **Settings panel** for parameters

---

#### UX Features

* Strategy summary cards
* Best strategy highlighting
* Inline parameter summary
* Clean dashboard layout

---

## 3. Core Capabilities

---

### 3.1 Data

* Load OHLC data
* Support multiple datasets
* (Future) support multiple timeframes

---

### 3.2 Simulation

* Execute strategies on shared dataset
* Track:

  * capital
  * positions
  * trades
  * equity curve

---

### 3.3 Evaluation

* Compute performance metrics
* Compare vs Buy & Hold
* Rank strategies
* Support relative performance metrics

---

### 3.4 Visualization

* Market context (candles)
* Performance (equity)
* Risk (drawdown)
* Trade inspection

---

### 3.5 Interaction

* Run simulations from UI
* Adjust parameters
* Highlight strategies
* Explore results visually

---

## 4. Architecture Principles

* Backend = **source of truth**
* UI = **visualization + interaction**
* Avoid logic duplication
* Prefer clarity over abstraction
* Build incrementally toward flexibility

---

## 5. Roadmap

---

### Phase 1 — Visualization ✅

* Charts (price, equity, drawdown)
* Strategy comparison
* Trade markers

---

### Phase 2 — Interaction 🚧

#### Completed

* Run via API
* Dataset selection
* Strategy highlighting
* Parameter controls (MA + RSI)
* Collapsible settings panel
* Time range controls (chart zoom)

---

#### Remaining

* [ ] Strategy enable/disable (API-driven)
* [ ] Timeframe selection (1d, 4h, etc.)

---

---

### ✨ Phase 2.5 — Realism & Execution (NEW)

**Goal:**

> Make backtests reflect real trading conditions

---

#### Execution Model

* [ ] Execute trades on **next candle open**
* [ ] Remove lookahead bias

---

#### Trading Frictions

* [ ] Add trading fees (e.g. 0.1%)
* [ ] Add slippage (simple model)

---

#### Engine Improvements

* [ ] Separate **signal vs execution**
* [ ] Ensure consistent fill logic

---

---

### Phase 3 — Strategy Research

**Goal:**

> Improve and evaluate strategies meaningfully

---

* [ ] Hybrid strategies (e.g. MA + RSI)
* [ ] Parameter sweeps
* [ ] Multi-run comparison
* [ ] Alternative scoring models

---

---

### Phase 4 — Portfolio Simulation

* [ ] Combine strategies
* [ ] Allocate capital
* [ ] Portfolio metrics

---

---

### Phase 5 — Advanced

* [ ] Multi-dataset comparison
* [ ] Strategy optimization
* [ ] Risk vs return visualization
* [ ] Drawdown analytics

---

## 6. API Design

---

### Current

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

---

### Future (Target)

```json
{
  "dataset": "BTCUSDT",
  "interval": "1d",
  "strategies": [
    { "type": "moving_average", "short": 10, "long": 50 },
    { "type": "rsi", "period": 14, "overbought": 70, "oversold": 30 }
  ]
}
```

---

## 7. Immediate Next Steps (Prioritized)

---

### 🔥 1. Engine Realism (Highest Priority)

* Add fees
* Implement next-candle execution
* Add slippage

---

### 🧠 2. Strategy Evolution

* Build hybrid strategy (MA + RSI)

---

### 🧩 3. API Evolution

* Strategy enable/disable
* Move toward structured config

---

### 🎛 4. UI Improvements

* Strategy toggles
* Parameter presets
* Better chart controls

---

## 8. Key Insights

* Profit alone is misleading
* Drawdown defines survivability
* Backtest realism is critical
* Indicators are building blocks, not strategies
* Visualization should answer questions

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

And:

> results are **credible, not just plausible**

---
