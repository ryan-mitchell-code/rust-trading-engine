# Trading Strategy Backtesting & Visualization System (PRD)

**This file is the product source of truth** for shipped behavior, API, and roadmap.

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
  * Trade lifecycle (BUY → SELL, plus forced close on last bar if still open)
  * Capital tracking (mark-to-market each bar)
* Execution model:

  * Strategies emit a **signal** on each bar from that bar’s OHLC (and history). **Fills are deferred:** a signal produced on bar *t* is applied at bar *t+1*’s **open** (via `PendingSignal` → `execute_signal`), not at the signal bar’s close—this avoids same-bar-close lookahead.
  * On the **final** bar, a signal is still **stored** but **not executed** (no later open); if `verbose` is on, the engine logs that the pending signal was dropped. An open position is **force-closed** at the **last bar’s close**.
  * **Fees:** `BacktestParams.fee_rate` charges a fraction of notional on **buy** (on cash allocated) and **sell** (on proceeds); PnL and metrics use post-fee amounts. Arena / `POST /run` use defaults (`fee_rate: 0` unless changed in code paths).
  * Position size: fixed fraction of **cash at entry** (10% of cash per buy, before fees).

---

#### Strategies

Arena runs a **fixed set** of four strategies every request (not yet selectable per run):

* Moving average crossover (trend-following; name includes window params, e.g. `moving_average_10_50`)
* RSI mean-reversion (`RSI`)
* Random (`random`)
* Buy & Hold (`buy_and_hold`) — benchmark for relative return

---

#### Metrics

* Return %, drawdown % (peak-to-trough ratio as percent)
* Sharpe ratio (per **bar** simple returns on the equity curve; **not** annualized)
* Max drawdown, max drawdown duration (consecutive bars below peak)
* Win rate, avg PnL (per completed round-trip as recorded by the engine)
* Relative return vs Buy & Hold (return % delta)
* Composite **score** (arena): normalized Sharpe, return, and drawdown components with fixed weights; results are **sorted by score** in the API response

---

#### Data

* Binance spot klines → full OHLC per bar
* `load_from_binance` reads **`outputs/binance_cache_<symbol>_<interval>_<limit>.json`** when present; on miss, fetches Binance and writes that file
* Default fetch limit: **1000** bars (`POST /run`)

---

#### API

* `POST /run`
* JSON body:

  * `dataset` (symbol, e.g. `BTCUSDT`)
  * `interval` (Binance kline interval, e.g. `1d`, `4h`)
  * `ma_short`, `ma_long`, `rsi_period`, `rsi_overbought`, `rsi_oversold`
* Request is mapped to [`RunConfig`](../../backend/src/config.rs) (MA + RSI only; strategy set is not configurable yet).

---

### Frontend (React + TypeScript)

---

#### Visualization

* Candlestick chart (OHLC)
* Equity curves (multi-strategy)
* Drawdown chart
* Trade markers on the price chart when a strategy is focused

---

#### Interaction

* Run backtest via API
* Dataset selection (header): `BTCUSDT`, `ETHUSDT`
* Strategy **focus** (cards + table): highlights one strategy for equity/drawdown/trade markers; does **not** change which strategies the backend runs
* Chart/table toggle
* Time range presets on the **candlestick** chart (`timeScale().setVisibleRange` via shared helpers)
* Collapsible **Settings** panel for MA and RSI parameters

---

#### UX

* Strategy summary cards; table includes **Score** column (matches API sort order)
* **“Best” card styling** uses **highest `return_pct`**, not composite score (may differ from top row in the table)
* Inline parameter summary when settings are collapsed
* Dashboard layout as implemented

---

## 3. Core Capabilities

---

### 3.1 Data

* Load OHLC data
* Support multiple datasets (via UI + API symbol string)
* UI currently sends a **fixed** interval (`1d` in code); changing bar size requires wiring interval through the UI to match `POST /run`

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
* Rank strategies (composite score + sort)
* Relative performance metrics

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
* Focus / highlight strategies for charts
* Explore results visually

---

## 4. Architecture Principles

* Backend = **source of truth**
* UI = **visualization + interaction**
* Avoid logic duplication
* Prefer clarity over abstraction
* Build incrementally toward flexibility
* **Extend execution and costs in one place** (engine / fill path) so strategies stay signal-only and tests can pin fill semantics

---

## 5. Roadmap

---

### Phase 1 — Visualization

* Charts (price, equity, drawdown)
* Strategy comparison
* Trade markers

---

### Phase 2 — Interaction

#### Done

* Run via API
* Dataset selection
* Strategy **focus** for charts and markers
* Parameter controls (MA + RSI)
* Collapsible settings panel
* Candlestick time-range presets (zoom)

#### Still open (not blocking Phase 2.5)

* [ ] Strategy **enable/disable** (request-driven: skip strategies in arena + reflect in UI)
* [ ] **Interval** selection in UI (pass through to `POST /run`; keep chart time scale in sync)

---

### Phase 2.5 — Realism & execution

**Goal:** make backtests closer to live trading and remove obvious lookahead.

#### Execution

* [x] Execute entries/exits on **next bar’s open** (deferred signal from prior bar), not the signal bar’s close
* [x] Document and test bar indexing so signal and fill cannot use future OHLC (see `engine.rs` tests: next-bar timing, final-bar drop, forced close)

#### Frictions

* [x] Trading fees (configurable **`fee_rate`** per side on buy allocation and sell proceeds)
* [ ] Slippage (simple model first: fixed bps or fixed tick off price)

#### Engine structure

* [x] Keep **signal generation** (strategies) separate from **fill / fee** application (`execute_signal`, pending queue)
* [x] Single, well-tested code path for “apply deferred signal → position / cash / trade log”
* [x] Regression tests: known candle series → expected fills, fees, and equity steps

---

### Phase 3 — Strategy research

**Goal:** improve and evaluate strategies meaningfully.

* [ ] Hybrid strategies (e.g. MA + RSI)
* [ ] Parameter sweeps
* [ ] Multi-run comparison
* [ ] **Scoring:** composite score exists with fixed weights; add configurability or alternative formulas if needed

---

### Phase 4 — Portfolio simulation

* [ ] Combine strategies
* [ ] Allocate capital
* [ ] Portfolio metrics

---

### Phase 5 — Advanced

* [ ] Multi-dataset comparison
* [ ] Strategy optimization
* [ ] Richer risk vs return views (beyond current equity + drawdown charts)
* [ ] Deeper drawdown analytics

---

## 6. API design

---

### Current (`POST /run`)

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

### Future (target)

Structured strategy list and run options (fees, slippage, enabled ids), for example:

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

## 7. Immediate next steps (prioritized)

1. **Phase 2.5 remainder:** slippage model; expose **`fee_rate`** (and later slippage) on **`POST /run`** when product wants tunable runs without code changes.
2. **Phase 2 gaps (when useful):** interval in UI; optional strategy toggles on the request body.
3. **Strategy evolution:** hybrid MA + RSI (execution model is now stable enough for comparable backtests).
4. **API evolution:** structured `strategies` array and run metadata; align UI “best” highlight with chosen rank key (return vs score) if product wants consistency.

---

## 8. Key insights

* Profit alone is misleading
* Drawdown defines survivability
* Backtest realism is critical
* Indicators are building blocks, not strategies
* Visualization should answer questions

---

## 9. Success criteria

A user can:

* select dataset
* configure strategy parameters (MA / RSI today)
* run backtest
* understand results visually

The system clearly communicates:

* performance
* risk
* trade-offs

And:

> results are **credible, not just plausible**

---
