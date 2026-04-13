# Trading handbook (rust-trader)

This handbook does **two** jobs:

1. **Teach trading and backtesting from scratch** — sections build in order (no prior markets background assumed).
2. **Anchor vocabulary to this codebase** — how we use each idea in Rust, the API, and the UI (with a compact lookup in section 8).

If you are new, read **sections 1–6 in order** once. If you already know markets, skip to **section 7** (code map) and use **section 8** as a cheat sheet. We will extend this document as execution and portfolio features get more realistic.

---

## Contents

- [1. Markets and prices (start here)](#1-markets-and-prices-start-here)
- [2. Time and bars: how history is sliced](#2-time-and-bars-how-history-is-sliced)
- [3. Spot trading: cash, positions, and PnL](#3-spot-trading-cash-positions-and-pnl)
- [4. Strategies, indicators, and signals](#4-strategies-indicators-and-signals)
- [5. Backtesting: simulating the past honestly](#5-backtesting-simulating-the-past-honestly)
- [6. How we measure “good” (metrics)](#6-how-we-measure-good-metrics)
- [7. How this repository fits together](#7-how-this-repository-fits-together)
- [8. Quick reference](#8-quick-reference)
- [Not modeled yet](#not-modeled-yet)

---

## 1. Markets and prices (start here)

### What you are looking at

A **market** is where people trade an **asset** (here: crypto **spot**—you buy real coins with **cash**, e.g. USDT). A **pair** (like **BTCUSDT**) means “Bitcoin priced in Tether.”

A **price** is whatever buyers and sellers agreed on in actual trades. You never see every trade in this app at once; you see **aggregated** history as **bars** (see section 2).

### Candlesticks in one minute

Each **bar** (candle) summarizes one time window (e.g. one day):

| Field | Meaning in plain English |
|--------|---------------------------|
| **Open** | First traded price (or official open) in that window |
| **High** | Highest price in the window |
| **Low** | Lowest price in the window |
| **Close** | Last price in the window (very often used for “where did we end up?”) |

The **wick** (thin line) spans low–high; the **body** spans open–close. Color shows whether close was above or below open. You do not need chart artistry to use this project: the **engine** mostly cares about **close** for valuation today; the **UI** draws full OHLC for context.

### Why four prices matter

**Close** is a simple default for “value now.” **High/low** matter if you care about **stops**, **intrabar** pain, or realistic fills (future work). This codebase’s `Candle` type stores **timestamp + OHLC**; execution rules decide which price is used for **fills** and **mark-to-market**.

---

## 2. Time and bars: how history is sliced

### Bar / candle

One **row** of history for a fixed **interval** (timeframe). The simulation walks bars **oldest → newest** like replaying a tape.

### Interval (timeframe)

Bar size: **`1d`** (daily), **`4h`**, **`1h`**, etc. Binance calls these **kline intervals**. The backend `POST /run` accepts `interval`; the UI may still hardcode one value—check `ui` when in doubt.

### Kline

Exchange API word for one candle row (Binance **`/api/v3/klines`**). Same idea as “bar” or “candle” here.

### Dataset / symbol

Which pair to load, e.g. **`BTCUSDT`**. Maps to the exchange **symbol** string.

### OHLCV

Standard column set: open, high, low, close, **volume**. Volume is **not** stored on our `Candle` type yet; we can add it when strategies need it.

---

## 3. Spot trading: cash, positions, and PnL

This project models **long-only spot**: you can hold **cash**, buy **units** of the asset, or be **flat** (no units). There is **no short** (betting on price down) and **no leverage** in the engine today.

### Cash

Money (e.g. USDT) not currently used to hold the asset.

### Long / open position

You own **size** units bought at some **entry** price. Until you sell, gains/losses are **unrealized**; we still **mark** the position to a current price each bar so **equity** moves with the market.

### Flat

**No** open position: everything is cash (for our engine).

### Mark-to-market

Revalue the open position every bar using a **mark price** (here: bar **close**) so **equity = cash + size × mark_price**. That is how the **equity curve** wiggles even before you sell.

### Buy and sell (engine rules)

- **Buy signal** only **opens** if you are **flat** (one position at a time).
- **Sell signal** only **closes** if you are **long**.
- If the last bar passes and you are still long, the engine **forces a final sell** so the run ends flat for accounting.

### Allocation and position fraction

On each buy, the engine spends a **fraction of current cash** (default **10%** via `BacktestParams.position_fraction`). That is **not** “10% of total equity” unless your cash happens to equal equity. Smaller fractions mean smaller bets and smoother equity.

### Realized PnL

When you **sell**, profit or loss **locks in**:

`pnl = proceeds - cash_you_put_in`  
with `proceeds = size × exit_price` in the current simple model.

### Round trip

One full **buy then sell** (or forced exit). **Win rate** and **average PnL** count **completed** round trips.

### OpenPosition (code)

Rust struct in `engine.rs`: **entry price**, **size**, **allocation** (cash spent at entry). Keeps state explicit for exits and logging.

---

## 4. Strategies, indicators, and signals

### Strategy

A **rule set** that, each bar, outputs **Buy**, **Sell**, or **Hold**. In code, types implement the **`Strategy`** trait: `next(&Candle) -> Signal`.

### Signal vs execution (important)

A **signal** is **intent** (“I want to buy”). **Execution** is **what actually happens**: which **price**, which **moment**, **fees**, **slippage**. This project is moving toward **clearer** separation so backtests stay honest (see section 5).

### Indicator

A number or series **derived** from prices (e.g. moving average at this bar). Indicators **feed** strategies; they are not a strategy by themselves.

### Moving average (MA)

Average of the last *N* **closes** (here). **Short** window: reacts fast. **Long** window: smooth, slow. Used to detect **trend**.

### Crossover (trend-following)

**Short MA crosses above long MA** → buy. **Short crosses below long** → sell. Implemented as **edge detection** (the **moment** of crossing), not “short is above long every bar.”

### RSI (Relative Strength Index)

A bounded oscillator (roughly **0–100**) measuring recent up vs down movement. **Mean-reversion** style: **buy** when RSI is **low** (oversold, e.g. 30), **sell** when **high** (overbought, e.g. 70). Parameters are on the API and in the UI settings.

### Mean reversion vs trend following

- **Mean reversion:** “Too cheap, buy; too rich, sell”—works in **range** markets; painful in strong trends.
- **Trend following:** “Strength begets strength”—ride moves; **whipsaws** in choppy markets.

Both are **ideas**, not guarantees. Backtests show **historical** fit, not future truth.

### Buy and hold (benchmark)

Buy once when the strategy allows, then **stay invested**. Used here as a **baseline**: “Did timing add value vs just holding?” **Relative return** compares your return % to this benchmark in the same run.

### Random strategy

A **noise** baseline: not meant to win—helps sanity-check whether a “real” edge is meaningful.

### Arena strategies (today)

Each API run executes a **fixed set** of strategies on the **same** bars: moving average (named with windows), RSI, random, buy-and-hold. You **focus** one in the UI for charts; you do **not** yet disable them on the server.

---

## 5. Backtesting: simulating the past honestly

### What backtesting is

**Replay** historical bars with fixed rules and measure **PnL**, **drawdowns**, and other stats. It answers: “If I had traded this way **in the past**, what would have happened?” It does **not** prove future performance.

### Why execution assumptions dominate

Tiny changes (fill price, fees, slippage) swing results. A model that fills at the **best** possible price inside each bar often **flatters** the strategy.

### Same-bar execution (current engine)

Signal and fill both use the **current bar’s close**. That implies you **knew** the close when you decided—usually **false** in real life unless you trade **after** the bar closes. That is a form of **lookahead bias** and tends to **inflate** backtest results.

### Next-bar execution (roadmap)

Decide on bar *t*, fill on bar *t+1* (often at **open**). Stricter and usually **worse** returns, but more **credible** for “end of day” style rules.

### Lookahead bias (general)

Using **future** information at a **past** decision time. Besides same-bar close, other examples elsewhere in finance: **survivorship bias** (only stocks that still exist), **data snooping** (tuning on the same data you test). Stay skeptical of **too-perfect** curves.

### Fees and commission

A **percent** or fixed cost per trade. They **compound** against you and should live in the **fill path** so every strategy pays the same rules.

### Slippage

You hoped for price *X*, got *Y* (worse). Modeled simply as bps off mid/close in many backtesters; real markets vary with **liquidity** and **speed**.

### Bid, ask, spread

**Bid:** best buy order. **Ask:** best sell order. **Spread:** gap between them. Filling at **close** ignores spread; more realistic models often **pay** half-spread or worse on market orders.

---

## 6. How we measure “good” (metrics)

### Equity curve

Account **equity** at **each bar**—the main **storyline** of a run. Feeds charts and several statistics.

### Return %

Percent gain or loss from **start** to **end** equity for that strategy run.

### Drawdown (pain)

From a **peak**, how far equity **fell** as a fraction of that peak. **Max drawdown** is the **worst** such drop in the run—answers “how bad did it get?”

### Drawdown duration

How long (in **bars**) equity stayed **under** the running peak before making a **new** high. Long underwater periods are hard to stick with psychologically.

### Drawdown curve (chart series)

Per-bar **underwater** plot vs **running peak**: values ≤ 0, 0 at new highs. Computed in **`equity_curve::drawdown_curve_from_equity`**. The UI should use the **backend** series for consistency.

### Sharpe ratio (this project)

**Idea:** reward per unit of **bumpiness** in returns—not just “made money.” **Implementation:** take **simple returns** between consecutive equity points `(E_t - E_{t-1}) / E_{t-1}`, then **mean / sample standard deviation**, risk-free = **0**. **Not annualized**; bar spacing defines the period. Do not compare Sharpe across **different** intervals without care.

### Win rate

Share of **completed** trades that had **positive** realized PnL.

### Average PnL

Average **realized** PnL per completed trade.

### Relative return vs buy and hold

**Your** return % **minus** buy-and-hold return % in the **same** backtest. Positive means you **beat** holding through the sample.

### Composite score (arena)

After all strategies finish, the backend **normalizes** Sharpe, return, and drawdown across them and forms a **weighted score** to **sort** results. The UI table order follows this. The **“best” card highlight** uses **highest return %** instead—those two can **disagree**; know which you are looking at.

---

## 7. How this repository fits together

| Piece | Role |
|--------|------|
| **`strategy/`** | Each strategy implements **`Strategy`** and emits **`Signal`**. |
| **`engine.rs`** | Walks candles, applies **execution** rules, tracks **cash/position**, builds **equity curve** and **trades**. |
| **`metrics.rs`** | **Incremental** stats while stepping (trades, peak drawdown, duration streaks). |
| **`equity_curve.rs`** | **Pure math** on finished equity: **Sharpe**, **drawdown series**. |
| **`arena.rs`** | Runs **all** default strategies, CSV export, **relative return**, **scoring**, **sort**. |
| **`api.rs`** | HTTP **`POST /run`** → load data → arena → JSON for the UI. |
| **UI** | Charts, tables, **focus** one strategy, parameters for MA/RSI. |

**`RunConfig`:** MA + RSI parameters from the API.  
**`BacktestParams`:** starting capital and position fraction (and later fees/slippage).  
**`BacktestRun`:** one shared **`market`** array plus **`results`** per strategy.

---

## 8. Quick reference

Alphabetical **reminders**; full intuition is in the sections above.

| Term | One-line reminder |
|------|-------------------|
| **Allocation** | Cash spent to open a position; also “position fraction” × cash for new buys. |
| **Arena** | Runs every default strategy on the same data; scores and sorts. |
| **Ask / bid** | Best sell / buy prices; **spread** is ask − bid. |
| **Backtest** | Simulate rules on historical bars. |
| **BacktestParams** | Engine: initial capital, position fraction (`engine.rs`). |
| **BacktestRun** | API JSON: `market` + per-strategy `results`. |
| **Bar / candle** | One interval of OHLC (+ volume elsewhere). |
| **Benchmark** | Here: **buy and hold** for comparison. |
| **Crossover** | Short MA crosses long → trade signal (edge-based). |
| **Dataset** | Symbol string, e.g. BTCUSDT. |
| **Drawdown** | Fall from a peak; **max** is worst episode. |
| **Equity** | Cash + marked position value. |
| **Equity curve** | Equity each bar. |
| **Execution** | Turning signals into fills (price, time, costs). |
| **Fee** | Trading cost as % or fixed. |
| **Flat** | No open position. |
| **Interval** | Bar size (1d, 4h, …). |
| **Indicator** | Derived from prices (MA, RSI, …). |
| **Kline** | Exchange API candle row. |
| **Long** | Holding units of the asset. |
| **Lookahead bias** | Using information not yet knowable at decision time. |
| **Mark-to-market** | Revalue open position each bar. |
| **Mean reversion** | Buy dips, sell rips (RSI-style). |
| **Moving average** | Rolling average of closes. |
| **Next-bar execution** | Signal bar *t*, fill bar *t+1* (stricter). |
| **OHLC** | Open, high, low, close. |
| **Open position** | Long position with entry, size, allocation. |
| **PnL** | Profit/loss; **realized** on sell. |
| **Position fraction** | Fraction of **cash** used per buy. |
| **Relative return** | Strategy return % − buy-and-hold return %. |
| **RSI** | Oscillator for overbought/oversold-style rules. |
| **RunConfig** | API MA/RSI parameters (`config.rs`). |
| **Same-bar execution** | Signal and fill on same bar’s close (optimistic). |
| **Sharpe (here)** | Mean bar return / std on equity curve; not annualized. |
| **Signal** | Buy / Sell / Hold from a strategy. |
| **Slippage** | Fill worse than expected. |
| **Spot** | Own asset with cash; no leverage in this engine. |
| **Strategy** | Rules → signals each bar. |
| **Strategy focus (UI)** | Highlight one run for charts; does not stop other runs. |
| **Trend following** | Ride trends (MA crossover-style). |
| **Win rate** | % of winning **completed** trades. |

---

## Not modeled yet

Topics you may see in books or other tools but **not** in this engine until we add them:

- **Short selling**, **margin**, **leverage**, **perpetuals funding**
- **Options**, **multi-asset portfolio** allocation
- Rich **order types** (limit, stop, IOC)
- **Annualized** Sharpe / volatility unless we implement it explicitly

When you add a feature, add a short subsection under the right **learning** section above, a row in **section 8** if it is a new term, and a line in **section 7** if new code owns it.
