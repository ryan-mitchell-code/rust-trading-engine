use crate::equity_curve::{drawdown_curve_from_equity, sharpe_ratio_from_equity_curve};
use crate::metrics::Metrics;
use crate::models::{Candle, Signal};
use crate::paths;
use crate::strategy::Strategy;
use serde::Serialize;

/// Capital, sizing, and (later) execution costs for a backtest run.
#[derive(Debug, Clone, Copy)]
pub struct BacktestParams {
    pub initial_capital: f64,
    /// Fraction of **current** cash allocated on each buy (e.g. `0.10` → 10%).
    pub position_fraction: f64,
}

impl Default for BacktestParams {
    fn default() -> Self {
        Self {
            initial_capital: 10_000.0,
            position_fraction: 0.10,
        }
    }
}

/// Open position: fill price per unit, size in units, cash paid at entry.
#[derive(Debug, Clone, Copy)]
struct OpenPosition {
    entry_price: f64,
    size: f64,
    allocation: f64,
}

/// Deferred strategy output: `timestamp` is the **generation** bar (`candle.timestamp` when `signal` was produced).
#[derive(Debug)]
struct PendingSignal {
    signal: Signal,
    timestamp: String,
}

impl PendingSignal {
    fn new(candle: &Candle, signal: Signal) -> Self {
        Self {
            signal,
            timestamp: candle.timestamp.clone(),
        }
    }
}

fn signal_verb(signal: &Signal) -> &'static str {
    match signal {
        Signal::Buy => "BUY",
        Signal::Sell => "SELL",
        Signal::Hold => "HOLD",
    }
}

fn calculate_capital(cash: f64, position: &Option<OpenPosition>, mark_price: f64) -> f64 {
    let held = position.as_ref().map(|p| p.size).unwrap_or(0.0);
    cash + held * mark_price
}

fn realized_pnl(size: f64, exit_price: f64, allocation: f64) -> f64 {
    size * exit_price - allocation
}

/// Bar timestamps and OHLC for the run (one row per candle).
pub fn market_series(data: &[Candle]) -> Vec<(String, f64, f64, f64, f64)> {
    data.iter()
        .map(|c| (c.timestamp.clone(), c.open, c.high, c.low, c.close))
        .collect()
}

/// Full export: shared market context plus per-strategy results (no duplicated OHLC per strategy).
#[derive(Serialize)]
pub struct BacktestRun {
    pub market: Vec<(String, f64, f64, f64, f64)>,
    pub results: Vec<BacktestResult>,
}

#[allow(dead_code)]
#[derive(Clone, Serialize)]
pub struct ResultSummary {
    pub equity_csv: String,
    pub trades_csv: String,
    pub final_capital: f64,
    pub total_pnl: f64,
    pub total_trades: u32,
    pub win_rate: f64,
    pub avg_pnl: f64,
    pub peak_equity: f64,
    pub max_drawdown: f64,
    pub max_drawdown_duration: u32,
    pub return_pct: f64,
    pub relative_return: f64,
    pub drawdown_pct: f64,
    /// Per-period Sharpe from the equity curve (`mean / sample std dev` of step returns); not annualized. See [`crate::equity_curve::sharpe_ratio_from_equity_curve`].
    pub sharpe_ratio: f64,
    pub score_sharpe_component: f64,
    pub score_return_component: f64,
    pub score_drawdown_component: f64,
    pub score: f64,
}

#[derive(Serialize)]
pub struct BacktestResult {
    /// Strategy id for API/UI and CSV basenames (stable key for this run).
    pub name: String,
    pub summary: ResultSummary,
    /// Mark-to-market capital per bar, aligned by index with `BacktestRun.market`.
    pub equity_curve: Vec<f64>,
    /// Drawdown ratio per bar vs running peak; same length as `equity_curve`.
    pub drawdown_curve: Vec<f64>,
    pub trades: Vec<Vec<String>>,
}

impl Default for ResultSummary {
    fn default() -> Self {
        Self {
            equity_csv: String::new(),
            trades_csv: String::new(),
            final_capital: 0.0,
            total_pnl: 0.0,
            total_trades: 0,
            win_rate: 0.0,
            avg_pnl: 0.0,
            peak_equity: 0.0,
            max_drawdown: 0.0,
            max_drawdown_duration: 0,
            return_pct: 0.0,
            relative_return: 0.0,
            drawdown_pct: 0.0,
            sharpe_ratio: 0.0,
            score_sharpe_component: 0.0,
            score_return_component: 0.0,
            score_drawdown_component: 0.0,
            score: 0.0,
        }
    }
}

fn make_trade_row(
    trade_id: u32,
    timestamp: &str,
    side: &str,
    price: f64,
    pnl: f64,
    capital: f64,
) -> Vec<String> {
    vec![
        trade_id.to_string(),
        timestamp.to_string(),
        side.to_string(),
        format!("{:.6}", price),
        format!("{:.2}", pnl),
        format!("{:.2}", capital),
    ]
}

/// Apply a **deferred** signal at `fill_price` (e.g. current bar **open**). Log rows use `trade_timestamp`.
fn execute_signal(
    signal: Signal,
    fill_price: f64,
    trade_timestamp: &str,
    cash: &mut f64,
    position: &mut Option<OpenPosition>,
    open_trade_id: &mut Option<u32>,
    next_trade_id: &mut u32,
    metrics: &mut Metrics,
    params: &BacktestParams,
) -> (
    Option<(u32, String, f64)>,
    Option<(u32, String, f64, f64)>,
) {
    let mut buy_log: Option<(u32, String, f64)> = None;
    let mut sell_log: Option<(u32, String, f64, f64)> = None;

    match signal {
        Signal::Buy => {
            if position.is_none() {
                let allocation = *cash * params.position_fraction;

                if allocation > f64::EPSILON {
                    let trade_id = *next_trade_id;
                    *next_trade_id += 1;
                    *open_trade_id = Some(trade_id);

                    let size = allocation / fill_price;
                    *cash -= allocation;
                    *position = Some(OpenPosition {
                        entry_price: fill_price,
                        size,
                        allocation,
                    });

                    buy_log = Some((trade_id, trade_timestamp.to_string(), fill_price));
                }
            }
        }
        Signal::Sell => {
            if let Some(OpenPosition {
                size,
                allocation,
                ..
            }) = position.take()
            {
                let exit_price = fill_price;
                let proceeds = size * exit_price;
                let pnl = realized_pnl(size, exit_price, allocation);

                *cash += proceeds;
                metrics.record_trade(pnl);

                let trade_id = open_trade_id
                    .take()
                    .expect("sell should follow a logged buy");

                sell_log = Some((trade_id, trade_timestamp.to_string(), exit_price, pnl));
            }
        }
        Signal::Hold => {}
    }

    (buy_log, sell_log)
}

pub fn run<S: Strategy>(
    data: &[Candle],
    mut strategy: S,
    strategy_name: &str,
    params: &BacktestParams,
    verbose: bool,
) -> BacktestResult {
    let initial = params.initial_capital;
    let mut cash = initial;
    let mut position: Option<OpenPosition> = None;
    let mut open_trade_id: Option<u32> = None;
    let mut next_trade_id: u32 = 1;
    let mut metrics = Metrics::new(initial);
    let mut trade_rows: Vec<Vec<String>> = Vec::new();
    let mut equity_curve: Vec<f64> = Vec::with_capacity(data.len());
    let mut pending_signal: Option<PendingSignal> = None;

    for (i, candle) in data.iter().enumerate() {
        let is_last_bar = i + 1 == data.len();
        // First bar: `pending_signal` is always `None` — never execute before any signal exists.
        // Last bar: deferred fills need a *following* bar's open; there is none, so drop pending without executing.
        let execute_pending = i > 0 && !is_last_bar;

        let (buy_log, sell_log) = if execute_pending {
            if let Some(pending) = pending_signal.take() {
                if verbose {
                    println!(
                        "Executing {} signal from {} at {} open",
                        signal_verb(&pending.signal),
                        pending.timestamp,
                        candle.timestamp
                    );
                }
                execute_signal(
                    pending.signal,
                    candle.open,
                    &candle.timestamp,
                    &mut cash,
                    &mut position,
                    &mut open_trade_id,
                    &mut next_trade_id,
                    &mut metrics,
                    params,
                )
            } else {
                (None, None)
            }
        } else {
            if is_last_bar && pending_signal.is_some() && verbose {
                println!("Dropping pending signal on last bar (no next open for execution)");
            }
            let _ = pending_signal.take();
            (None, None)
        };

        pending_signal = Some(PendingSignal::new(candle, strategy.next(candle)));

        let capital = calculate_capital(cash, &position, candle.close);

        if let Some((trade_id, ref ts, price)) = buy_log {
            if verbose {
                println!(
                    "{} | id {} | BUY | price {:.6} | pnl 0.00 | capital {:.2}",
                    ts, trade_id, price, capital
                );
            }

            trade_rows.push(make_trade_row(
                trade_id,
                ts,
                "BUY",
                price,
                0.0_f64,
                capital,
            ));
        }
        if let Some((trade_id, ref ts, exit_price, pnl)) = sell_log {
            if verbose {
                println!(
                    "{} | id {} | SELL | price {:.6} | pnl {:.2} | capital {:.2}",
                    ts, trade_id, exit_price, pnl, capital
                );
            }

            trade_rows.push(make_trade_row(
                trade_id,
                ts,
                "SELL",
                exit_price,
                pnl,
                capital,
            ));
        }

        metrics.update_equity(capital);
        equity_curve.push(capital);
    }

    if let Some(OpenPosition {
        size,
        allocation,
        ..
    }) = position.take()
    {
        let last = data.last().expect("data not empty");
        let exit_price = last.close;
        let proceeds = size * exit_price;
        let pnl = realized_pnl(size, exit_price, allocation);

        cash += proceeds;
        metrics.record_trade(pnl);

        let trade_id = open_trade_id
            .take()
            .expect("final sell should follow a logged buy");
        let capital = calculate_capital(cash, &position, exit_price);

        if verbose {
            println!(
                "{} | id {} | SELL (final) | price {:.6} | pnl {:.2} | capital {:.2}",
                last.timestamp, trade_id, exit_price, pnl, capital
            );
        }

        trade_rows.push(make_trade_row(
            trade_id,
            &last.timestamp,
            "SELL",
            exit_price,
            pnl,
            capital,
        ));

        if let Some(last_row) = equity_curve.last_mut() {
            *last_row = capital;
        }
        metrics.update_equity(capital);
    }

    let last_close = data.last().map(|c| c.close).unwrap_or(0.0);
    let final_capital = calculate_capital(cash, &position, last_close);

    let sharpe_ratio = sharpe_ratio_from_equity_curve(&equity_curve);
    let drawdown_curve = drawdown_curve_from_equity(&equity_curve);
    let max_drawdown = metrics.max_drawdown();
    let return_pct = (final_capital - initial) / initial * 100.0;
    let drawdown_pct = max_drawdown * 100.0;

    let safe_name = strategy_name.replace(['/', '\\'], "_");
    let equity_path = paths::output_file(&format!("equity_{}.csv", safe_name))
        .to_string_lossy()
        .into_owned();
    let trades_path = paths::output_file(&format!("trades_{}.csv", safe_name))
        .to_string_lossy()
        .into_owned();

    BacktestResult {
        name: strategy_name.to_string(),
        summary: ResultSummary {
            equity_csv: equity_path,
            trades_csv: trades_path,
            final_capital,
            total_pnl: metrics.total_pnl(),
            total_trades: metrics.trades(),
            win_rate: metrics.win_rate(),
            avg_pnl: metrics.avg_pnl(),
            peak_equity: metrics.peak_equity(),
            max_drawdown,
            max_drawdown_duration: metrics.max_drawdown_duration(),
            return_pct,
            drawdown_pct,
            sharpe_ratio,
            ..Default::default()
        },
        equity_curve,
        drawdown_curve,
        trades: trade_rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = f64::EPSILON * 10.0;

    fn assert_close(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPS,
            "expected {:.12}, got {:.12}",
            b,
            a
        );
    }

    #[test]
    fn calculate_capital_no_position() {
        assert_close(calculate_capital(10_000.0, &None, 50.0), 10_000.0);
    }

    #[test]
    fn calculate_capital_mark_to_market() {
        let position = Some(OpenPosition {
            entry_price: 100.0,
            size: 2.0,
            allocation: 200.0,
        });
        // cash 8_000 + 2 units * mark 110 = 8_220
        assert_close(calculate_capital(8_000.0, &position, 110.0), 8_220.0);
    }

    #[test]
    fn realized_pnl_profit() {
        // 10 units @ exit 11, allocation 100 → proceeds 110, pnl +10
        assert_close(realized_pnl(10.0, 11.0, 100.0), 10.0);
    }

    #[test]
    fn realized_pnl_loss() {
        // 10 units @ exit 9, allocation 100 → pnl -10
        assert_close(realized_pnl(10.0, 9.0, 100.0), -10.0);
    }

    #[test]
    fn realized_pnl_breakeven() {
        assert_close(realized_pnl(5.0, 40.0, 200.0), 0.0);
    }

    /// One round-trip: open with 10% of cash, close at a higher price; no engine/strategy/IO.
    #[test]
    fn trade_lifecycle_buy_then_sell() {
        let params = BacktestParams::default();
        let mut cash = params.initial_capital;
        let entry_price = 2_000.0;
        let allocation = cash * params.position_fraction;
        let size = allocation / entry_price;
        cash -= allocation;
        let mut position: Option<OpenPosition> = Some(OpenPosition {
            entry_price,
            size,
            allocation,
        });

        assert_close(calculate_capital(cash, &position, entry_price), 10_000.0);

        let exit_price = 2_200.0;
        let (sz, alloc) = match position.take() {
            Some(p) => {
                assert_close(p.entry_price, entry_price);
                (p.size, p.allocation)
            }
            None => panic!("expected open position"),
        };

        let pnl = realized_pnl(sz, exit_price, alloc);
        let proceeds = sz * exit_price;
        cash += proceeds;

        assert_close(pnl, 100.0);
        assert_close(cash, 10_100.0);
        assert!(position.is_none());
        assert_close(calculate_capital(cash, &position, exit_price), 10_100.0);
    }
}
