use crate::equity_curve::{drawdown_curve_from_equity, sharpe_ratio_from_equity_curve};
use crate::metrics::Metrics;
use crate::models::{Candle, Signal};
use crate::paths;
use crate::strategy::Strategy;
use serde::Serialize;

/// Capital, sizing, and execution costs for a backtest run.
#[derive(Debug, Clone, Copy)]
pub struct BacktestParams {
    pub initial_capital: f64,
    /// Fraction of **current** cash allocated on each buy (e.g. `0.10` → 10%).
    pub position_fraction: f64,
    /// Per-side fee as a fraction of notional (e.g. `0.001` = 0.1%). Charged on buy allocation and on sell proceeds.
    pub fee_rate: f64,
}

impl Default for BacktestParams {
    fn default() -> Self {
        Self {
            initial_capital: 10_000.0,
            position_fraction: 0.10,
            fee_rate: 0.0,
        }
    }
}

/// Open position: fill price per unit, size in units, cash allocated to the position, and entry fee paid.
#[derive(Debug, Clone, Copy)]
struct OpenPosition {
    entry_price: f64,
    size: f64,
    allocation: f64,
    buy_fee: f64,
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
                    let buy_fee = allocation * params.fee_rate;
                    let cash_out = allocation + buy_fee;
                    if *cash + f64::EPSILON >= cash_out {
                        let trade_id = *next_trade_id;
                        *next_trade_id += 1;
                        *open_trade_id = Some(trade_id);

                        let size = allocation / fill_price;
                        *cash -= cash_out;
                        *position = Some(OpenPosition {
                            entry_price: fill_price,
                            size,
                            allocation,
                            buy_fee,
                        });

                        buy_log = Some((trade_id, trade_timestamp.to_string(), fill_price));
                    }
                }
            }
        }
        Signal::Sell => {
            if let Some(OpenPosition {
                size,
                allocation,
                buy_fee,
                ..
            }) = position.take()
            {
                let exit_price = fill_price;
                let proceeds = size * exit_price;
                let sell_fee = proceeds * params.fee_rate;
                let net_proceeds = proceeds - sell_fee;
                let pnl = net_proceeds - allocation - buy_fee;

                *cash += net_proceeds;
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
        // First bar: no prior signal to execute. Later bars: run the *previous* bar's deferred signal at this bar's open
        // (including the final bar — then any open position is force-closed at `data.last().close` after the loop).
        let execute_pending = i > 0;

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
            // No prior bar to execute against; clear before this bar's signal is stored (always overwritten on the next line).
            #[allow(unused_assignments)]
            {
                pending_signal = None;
            }
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

    // Last bar's strategy output is never executed (no following bar open). Discard for clarity; no trades/metrics/cash impact.
    if let Some(pending) = pending_signal.take() {
        if verbose {
            println!(
                "Dropping pending {} signal from {} (no next open for execution)",
                signal_verb(&pending.signal),
                pending.timestamp
            );
        }
    }

    if let Some(OpenPosition {
        size,
        allocation,
        buy_fee,
        ..
    }) = position.take()
    {
        let last = data.last().expect("data not empty");
        let exit_price = last.close;
        let proceeds = size * exit_price;
        let sell_fee = proceeds * params.fee_rate;
        let net_proceeds = proceeds - sell_fee;
        let pnl = net_proceeds - allocation - buy_fee;

        cash += net_proceeds;
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
    use crate::strategy::Strategy;

    /// Deterministic signal sequence for engine tests (`next` advances each bar).
    struct TestStrategy {
        signals: Vec<Signal>,
        index: usize,
    }

    impl TestStrategy {
        fn new(signals: Vec<Signal>) -> Self {
            Self { signals, index: 0 }
        }
    }

    impl Strategy for TestStrategy {
        fn next(&mut self, _candle: &Candle) -> Signal {
            let out = self
                .signals
                .get(self.index)
                .copied()
                .unwrap_or(Signal::Hold);
            self.index += 1;
            out
        }
    }

    #[test]
    fn test_strategy_emits_scripted_signals_then_hold() {
        use crate::models::Candle;
        let bar = Candle::test_close(100.0);
        let mut strat = TestStrategy::new(vec![Signal::Buy, Signal::Sell]);
        assert_eq!(strat.next(&bar), Signal::Buy);
        assert_eq!(strat.next(&bar), Signal::Sell);
        assert_eq!(strat.next(&bar), Signal::Hold);
        assert_eq!(strat.next(&bar), Signal::Hold);
    }

    /// Buy signaled on bar 0 fills at bar 1 open (110), not bar 0 close (100).
    #[test]
    fn next_bar_buy_executes_at_following_open() {
        use crate::models::Candle;
        let candles = vec![
            Candle {
                timestamp: "bar0".into(),
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
            },
            Candle {
                timestamp: "bar1".into(),
                open: 110.0,
                high: 110.0,
                low: 110.0,
                close: 110.0,
            },
            Candle {
                timestamp: "bar2".into(),
                open: 120.0,
                high: 120.0,
                low: 120.0,
                close: 120.0,
            },
        ];
        let strat = TestStrategy::new(vec![Signal::Buy, Signal::Hold, Signal::Hold]);
        let res = run(
            &candles,
            strat,
            "next_bar_timing",
            &BacktestParams::default(),
            false,
        );
        let buy = res
            .trades
            .iter()
            .find(|row| row.get(2).is_some_and(|s| s == "BUY"))
            .expect("BUY row");
        assert_eq!(buy[1], "bar1", "fill timestamp should be execution bar");
        let fill_price: f64 = buy[3].parse().expect("price column");
        assert_close(fill_price, 110.0);
    }

    /// Buy on bar 0 fills at bar 1 open (200), not signal bar close (150).
    #[test]
    fn next_bar_buy_uses_next_open_not_signal_close() {
        use crate::models::Candle;
        let candles = vec![
            Candle {
                timestamp: "b0".into(),
                open: 100.0,
                high: 150.0,
                low: 100.0,
                close: 150.0,
            },
            Candle {
                timestamp: "b1".into(),
                open: 200.0,
                high: 200.0,
                low: 200.0,
                close: 200.0,
            },
        ];
        let strat = TestStrategy::new(vec![Signal::Buy]);
        let res = run(
            &candles,
            strat,
            "next_open_not_close",
            &BacktestParams::default(),
            false,
        );
        let buy = res
            .trades
            .iter()
            .find(|row| row.get(2).is_some_and(|s| s == "BUY"))
            .expect("BUY row");
        let fill_price: f64 = buy[3].parse().expect("price column");
        assert_close(fill_price, 200.0);
    }

    /// Deferred execution has no following open on the final bar — Buy there is dropped.
    #[test]
    fn last_bar_signal_is_not_executed() {
        use crate::models::Candle;
        let candles = vec![
            Candle {
                timestamp: "b0".into(),
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
            },
            Candle {
                timestamp: "b1".into(),
                open: 110.0,
                high: 110.0,
                low: 110.0,
                close: 110.0,
            },
        ];
        let strat = TestStrategy::new(vec![Signal::Hold, Signal::Buy]);
        let res = run(
            &candles,
            strat,
            "last_bar_no_exec",
            &BacktestParams::default(),
            false,
        );
        assert!(
            res.trades.is_empty(),
            "expected no trades when Buy only appears on the last bar"
        );
    }

    /// Open position at last bar's open is force-closed at that bar's close.
    #[test]
    fn open_position_force_closed_at_final_bar_close() {
        use crate::models::Candle;
        let candles = vec![
            Candle {
                timestamp: "b0".into(),
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
            },
            Candle {
                timestamp: "b1".into(),
                open: 110.0,
                high: 120.0,
                low: 110.0,
                close: 120.0,
            },
        ];
        let strat = TestStrategy::new(vec![Signal::Buy]);
        let res = run(
            &candles,
            strat,
            "force_close_final_bar",
            &BacktestParams::default(),
            false,
        );
        let buys: Vec<_> = res
            .trades
            .iter()
            .filter(|row| row.get(2).is_some_and(|s| s == "BUY"))
            .collect();
        let sells: Vec<_> = res
            .trades
            .iter()
            .filter(|row| row.get(2).is_some_and(|s| s == "SELL"))
            .collect();
        assert_eq!(buys.len(), 1, "expected one BUY");
        assert_eq!(sells.len(), 1, "expected one SELL (final)");
        let buy_price: f64 = buys[0][3].parse().expect("BUY price");
        assert_close(buy_price, 110.0);
        let sell_price: f64 = sells[0][3].parse().expect("SELL price");
        assert_close(sell_price, 120.0);
        let pnl: f64 = sells[0][4].parse().expect("SELL pnl");
        assert!(pnl > 0.0, "expected positive PnL, got {}", pnl);
    }

    /// At flat price, round-trip still loses to entry + exit fees.
    #[test]
    fn fee_reduces_pnl_on_flat_price() {
        use crate::models::Candle;
        let candles = vec![
            Candle {
                timestamp: "b0".into(),
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
            },
            Candle {
                timestamp: "b1".into(),
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
            },
        ];
        let strat = TestStrategy::new(vec![Signal::Buy]);
        let params = BacktestParams {
            fee_rate: 0.01,
            ..BacktestParams::default()
        };
        let res = run(&candles, strat, "fee_flat_pnl", &params, false);
        let buys: Vec<_> = res
            .trades
            .iter()
            .filter(|row| row.get(2).is_some_and(|s| s == "BUY"))
            .collect();
        let sells: Vec<_> = res
            .trades
            .iter()
            .filter(|row| row.get(2).is_some_and(|s| s == "SELL"))
            .collect();
        assert_eq!(buys.len(), 1);
        assert_eq!(sells.len(), 1);
        assert_close(buys[0][3].parse::<f64>().expect("BUY price"), 100.0);
        assert_close(sells[0][3].parse::<f64>().expect("SELL price"), 100.0);
        let pnl: f64 = sells[0][4].parse().expect("SELL pnl");
        assert!(pnl < 0.0, "expected negative PnL from fees, got {}", pnl);
    }

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
            buy_fee: 0.0,
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
            buy_fee: 0.0,
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
