# Relative Strength Index (RSI) — Learning Notes

## Overview

The **Relative Strength Index (RSI)** is a momentum oscillator used to measure the speed and magnitude of recent price movements.

It helps answer:

> **Is the market overbought or oversold?**

RSI is bounded between **0 and 100** and is typically plotted as a separate line below the price chart.

---

## Key Levels

* **Above 70 → Overbought**

  * Market may be stretched upward
  * Potential for reversal or pullback

* **Below 30 → Oversold**

  * Market may be stretched downward
  * Potential for bounce or recovery

These levels are not strict signals — they indicate *conditions*, not guarantees.

---

## Formula

RSI is derived from the ratio of average gains to average losses over a given period.

```text
RSI = 100 - 100 / (1 + RS)
```

Where:

* RS = Average Gain / Average Loss

---

## Calculation (Conceptual)

1. Look at the last **N periods** (commonly 14)
2. Calculate:

   * Average gain (up moves)
   * Average loss (down moves)
3. Compute RS
4. Convert to RSI using the formula

---

## Interpretation

RSI is a **momentum indicator**, not a trend indicator.

### What it tells you:

* Strength of recent moves
* Potential exhaustion points
* Overextension in price

---

## Basic Strategy Logic

Simple RSI-based trading rules:

```text
BUY  when RSI < 30
SELL when RSI > 70
HOLD otherwise
```

This is a **mean-reversion strategy**:

* Buy weakness
* Sell strength

---

## Strengths

* Easy to understand and implement
* Works well in **range-bound markets**
* Highlights overextended conditions clearly
* Complements trend-following strategies (like moving averages)

---

## Weaknesses

* Can stay overbought/oversold for long periods in strong trends
* Generates false signals in trending markets
* Does not account for broader market structure
* Sensitive to parameter choice (period length)

---

## RSI vs Moving Average

| Feature  | RSI                 | Moving Average     |
| -------- | ------------------- | ------------------ |
| Type     | Momentum oscillator | Trend-following    |
| Signals  | Reversal-focused    | Trend continuation |
| Best in  | Sideways markets    | Trending markets   |
| Weakness | Trends              | Chop               |

---

## Practical Insights

* RSI < 30 does **not guarantee** immediate reversal
* RSI > 70 does **not mean price must drop**
* Context matters:

  * Trend strength
  * Market volatility
  * timeframe

---

## Common Variations

* **RSI (14)** — standard
* **RSI (2)** — very short-term (more signals, more noise)
* **RSI (21+)** — smoother, slower signals

---

## Advanced Concepts (Future)

* RSI divergence (price vs indicator mismatch)
* RSI trendlines
* Combining RSI with moving averages
* Adaptive thresholds (not fixed 30/70)

---

## Key Takeaway

> RSI helps identify **when a move may be overextended**,
> but not necessarily **when it will reverse**.

---

## How It Fits in This Project

In your system:

* RSI provides a **non-trend strategy**
* Complements Moving Average
* Helps compare:

  * trend-following vs mean-reversion
* Enables deeper strategy evaluation

---

## Next Questions

* Does RSI outperform MA in certain datasets?
* How sensitive is RSI to period length?
* Should thresholds (30/70) be adjusted per market?
* Can RSI be combined with trend filters?

---

## Summary

RSI is:

* simple
* powerful
* widely used

But most importantly:

> It teaches how **market conditions affect strategy performance**
