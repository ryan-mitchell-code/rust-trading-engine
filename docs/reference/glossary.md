# 📖 Glossary of Terms

This project uses common trading and simulation terms. Definitions are kept short and tied to how they are used in this codebase.

## 🕯️ Candle (Candlestick)

A single unit of market data over a time period (for example, 1 day).

Typical candle fields:

- Open: price at start of period
- High: highest price in period
- Low: lowest price in period
- Close: price at end of period
- Volume: total traded amount

In **this codebase**, the Rust `Candle` type stores **`timestamp`**, **`open`**, **`high`**, **`low`**, and **`close`** (OHLC). The engine still marks equity to **`close`** each bar; the UI uses full OHLC for the market candlestick chart. CSV/API loaders fill all fields where available.

## 📊 OHLCV

Shorthand for market data columns:

- Open
- High
- Low
- Close
- Volume

This is the standard historical format loaded by the backtester.

## 📈 Moving Average (MA)

The average price over a fixed number of periods.

- Short MA reacts faster to recent changes.
- Long MA is smoother and slower.

In this project, moving averages are used to generate crossover signals.

## 🔀 Crossover

A signal event when short and long moving averages cross:

- BUY: short MA crosses above long MA
- SELL: short MA crosses below long MA

The project uses crossover events (state change), not raw `short > long` comparisons.

## 📍 Position

Represents whether the engine currently has an open trade.

- `None`: flat (no open position)
- `Some((entry_price, size, allocation))`: open position with entry details

This structure keeps trade state explicit and avoids recomputing allocation at exit.

## 🟢 Buy Signal

A strategy output indicating entry.

In this project, a BUY is only executed when `position.is_none()`.

## 🔴 Sell Signal

A strategy output indicating exit.

In this project, a SELL is only executed when a position exists (`Some(...)`).

## 💰 PnL (Profit and Loss)

Realized trade result when closing a position.

Formula used in this project:

`pnl = proceeds - allocation`

where:

- `allocation`: capital committed at BUY
- `proceeds`: value received at SELL (`size * exit_price`)

## 💼 Capital / Equity

Total account value at a point in time.

In this project:

`equity = cash + position_value`

where `position_value = size * current_price` when a position is open.

## 📉 Backtesting

Running a strategy on historical data to evaluate behavior and risk before live trading.

## ⚖️ Strategy

A set of rules for producing BUY / SELL / HOLD signals.

In this project, strategies implement the `Strategy` trait and are run by the same engine for comparison.

## 📉 Drawdown

Drawdown measures how far equity falls from a peak to a later low.

It is a risk metric focused on loss depth during bad periods.

### Formula

`drawdown = (peak_equity - current_equity) / peak_equity`

### Types

**Max Drawdown**

The largest drawdown observed over the period. It represents the worst historical peak-to-trough decline.

**Current Drawdown**

The current drop from the most recent peak.

**Drawdown Duration**

How long it takes to recover from a drawdown back to a new peak.

### Why It Matters

- Measures risk, not return
- Shows how unstable a strategy can be
- Helps compare strategies beyond final profit
- Large drawdowns are harder to tolerate psychologically

### Example

If equity rises to 10,000 and then falls to 8,000:

`drawdown = (10,000 - 8,000) / 10,000 = 20%`

### Key Insight

A profitable strategy is not automatically a robust strategy.

Lower returns with smaller drawdowns can be easier to operate and often more reliable over time.

### In This Project

- Max drawdown is tracked in `Metrics`
- It is updated on each equity update via `update_equity(equity)`
- It is used to compare strategy risk profiles

### Per-bar drawdown series (`drawdown_curve`)

For charts, the engine also emits a **time series** of drawdown vs a **running peak** (not the same sign convention as max drawdown above):

`(current_equity - peak_equity) / peak_equity`

So it is **≤ 0** while underwater and **0** at new highs. This is serialized in JSON for the UI; **do not recompute drawdown in the front-end** from equity.

## 📐 Sharpe Ratio

The Sharpe ratio summarizes **risk-adjusted** return: how much return you get per unit of **volatility** in returns (not per unit of profit alone).

### Formula (conceptual)

`sharpe_ratio = mean_return / standard_deviation_of_returns`

This project uses a **simplified** form: the risk-free rate is treated as **zero**, and returns are built from the **equity curve** (see below).

### Interpretation

- **Higher** Sharpe → returns are more **consistent** relative to how much they bounce around.
- **Lower** Sharpe → returns are **more volatile** or **less stable** relative to their mean (including cases where the mean is small).

### Intuition

Two strategies can end with similar profit, but:

- **Smooth, steady** equity growth tends to produce a **higher** Sharpe (lower volatility of returns).
- **Large swings** in equity produce a **lower** Sharpe (higher volatility of returns).

So the Sharpe ratio **rewards steadiness** and **penalizes choppy** paths, not just the ending balance.

### Example

| Strategy | Avg return | Volatility (std dev) | Sharpe (illustrative) |
|----------|------------|----------------------|-------------------------|
| A        | 1%         | 0.5%                 | 2.0                     |
| B        | 1%         | 2%                   | 0.5                     |

Same average return, but **A** is preferable on a risk-adjusted basis because volatility is lower.

### Notes

- **Risk-free rate = 0** here (common simplification in small backtests).
- **Sensitive** to outliers and to how often you sample the equity curve (each bar is one observation).
- A **full** institutional Sharpe often **annualizes** returns and volatility; this codebase keeps a **simple per-period** ratio for clarity.

### In This Project

- Built from the **equity curve** collected during `engine::run`.
- **Simple returns** between consecutive points: `(E_t - E_{t-1}) / E_{t-1}` when the prior equity is positive.
- **Sample** standard deviation of those returns (when there are enough points); ratio is **0** if volatility is negligible or there are too few returns (safe division).
- Stored as `sharpe_ratio` on `ResultSummary` and shown in the **comparison table** in `main`.
- **Not annualized**; bar spacing defines the period.

### Key Insight

**Profit alone is not enough.** A strategy with stable, repeatable growth is often easier to rely on than one with higher but more **volatile** equity paths—Sharpe is one way to make that trade-off visible.

## 📦 Exported backtest (`BacktestRun`)

The JSON export (`outputs/results.json`) is a single object with:

- **`market`**: one series per bar, each row **`[timestamp, open, high, low, close]`** (from `engine::market_series`), shared by all strategies.
- **`results`**: array of per-strategy rows (name, summary, **equity curve**, **drawdown curve**, trades).

This avoids duplicating market data for every strategy row and keeps the UI aligned on one x-axis.
