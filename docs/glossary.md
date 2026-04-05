# 📖 Glossary of Terms

This project uses common trading and market data terminology. Here’s a quick reference:

## 🕯️ Candle (or Candlestick)

A single unit of market data over a time period (e.g. 1 day, 1 minute).

Each candle contains:

Open – price at the start of the period
High – highest price during the period
Low – lowest price during the period
Close – price at the end of the period
Volume – total traded amount during the period

## 📊 OHLCV

Shorthand for:

Open, High, Low, Close, Volume

This is the standard format for historical price data.

## 📈 Moving Average (MA)

An average of prices over a fixed number of periods.

Short MA → reacts quickly to price changes
Long MA → smoother, slower trend

Used to identify trends and generate signals.

## 🔀 Crossover

A signal generated when:

BUY → short MA crosses above long MA
SELL → short MA crosses below long MA

This project uses crossover logic instead of raw comparisons.

## 📍 Position

Represents whether the system currently holds an asset.
None → no active trade (flat)
Some(price) → holding a position entered at price

🟢 Buy Signal
Indicates the strategy suggests entering a trade.
In this system:
Only executed if no position is currently open

🔴 Sell Signal
Indicates the strategy suggests exiting a trade.
In this system:
Only executed if a position is currently open

## 💰 Profit (PnL)

Calculated as:

Profit = Sell Price - Buy Price

(Current implementation assumes 1 unit per trade and no fees.)

## 📉 Backtesting

Simulating a trading strategy on historical data to evaluate performance.

## ⚖️ Strategy

A set of rules that determines:

When to buy
When to sell

In this project:

Moving Average Crossover Strategy
