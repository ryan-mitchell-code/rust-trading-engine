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
