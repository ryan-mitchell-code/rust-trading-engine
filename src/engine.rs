use crate::csv::write_csv;
use crate::metrics::Metrics;
use crate::models::{Candle, Signal};
use crate::strategy::Strategy;

const INITIAL_CAPITAL: f64 = 10_000.0;
const POSITION_FRACTION: f64 = 0.10;

/// Open position: `(entry_price per unit, size in units, cash allocated at entry)`.
type Position = (f64, f64, f64);

fn calculate_capital(cash: f64, position: &Option<Position>, mark_price: f64) -> f64 {
    let held = position.as_ref().map(|(_, size, _)| *size).unwrap_or(0.0);
    cash + held * mark_price
}

fn realized_pnl(size: f64, exit_price: f64, allocation: f64) -> f64 {
    size * exit_price - allocation
}

fn sharpe_ratio_from_equity_curve(equity_curve: &[(String, f64)]) -> f64 {
    if equity_curve.len() < 2 {
        return 0.0;
    }

    let mut returns: Vec<f64> = Vec::with_capacity(equity_curve.len() - 1);

    for w in equity_curve.windows(2) {
        let prev = w[0].1;
        let curr = w[1].1;

        debug_assert!(prev > 0.0, "equity should always be positive");

        returns.push((curr - prev) / prev);
    }

    let n = returns.len();
    if n < 2 {
        return 0.0;
    }

    let mean = returns.iter().sum::<f64>() / n as f64;

    let variance = returns
        .iter()
        .map(|r| (r - mean).powi(2))
        .sum::<f64>()
        / (n as f64 - 1.0);

    let std_dev = variance.sqrt();

    if std_dev <= f64::EPSILON {
        0.0
    } else {
        mean / std_dev
    }
}

#[allow(dead_code)]
pub struct ResultSummary {
    pub strategy_name: String,
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
    pub sharpe_ratio: f64,
    pub score_sharpe_component: f64,
    pub score_return_component: f64,
    pub score_drawdown_component: f64,
    pub score: f64,
}

impl Default for ResultSummary {
    fn default() -> Self {
        Self {
            strategy_name: String::new(),
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

pub fn run<S: Strategy>(
    data: &[Candle],
    mut strategy: S,
    strategy_name: &str,
    verbose: bool,
) -> ResultSummary {
    let mut cash = INITIAL_CAPITAL;
    let mut position: Option<Position> = None;
    let mut open_trade_id: Option<u32> = None;
    let mut next_trade_id: u32 = 1;
    let mut metrics = Metrics::new(INITIAL_CAPITAL);
    let mut trade_rows: Vec<Vec<String>> = Vec::new();
    let mut equity_curve: Vec<(String, f64)> = Vec::with_capacity(data.len());

    for candle in data {
        let signal = strategy.next(candle.close);

        let mut buy_log: Option<(u32, String, f64)> = None;
        let mut sell_log: Option<(u32, String, f64, f64)> = None;

        match signal {
            Signal::Buy => {
                if position.is_none() {
                    let allocation = cash * POSITION_FRACTION;

                    if allocation > f64::EPSILON {
                        let trade_id = next_trade_id;
                        next_trade_id += 1;
                        open_trade_id = Some(trade_id);

                        let size = allocation / candle.close;
                        cash -= allocation;
                        position = Some((candle.close, size, allocation));

                        buy_log = Some((trade_id, candle.timestamp.clone(), candle.close));
                    }
                }
            }
            Signal::Sell => {
                if let Some((_, size, allocation)) = position.take() {
                    let exit_price = candle.close;
                    let proceeds = size * exit_price;
                    let pnl = realized_pnl(size, exit_price, allocation);

                    cash += proceeds;
                    metrics.record_trade(pnl);

                    let trade_id = open_trade_id
                        .take()
                        .expect("sell should follow a logged buy");

                    sell_log = Some((trade_id, candle.timestamp.clone(), exit_price, pnl));
                }
            }
            Signal::Hold => {}
        }

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
        equity_curve.push((candle.timestamp.clone(), capital));
    }

    if let Some((_, size, allocation)) = position.take() {
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
            *last_row = (last.timestamp.clone(), capital);
        }
        metrics.update_equity(capital);
    }

    let last_close = data.last().map(|c| c.close).unwrap_or(0.0);
    let final_capital = calculate_capital(cash, &position, last_close);

    let sharpe_ratio = sharpe_ratio_from_equity_curve(&equity_curve);
    let max_drawdown = metrics.max_drawdown();
    let return_pct = (final_capital - INITIAL_CAPITAL) / INITIAL_CAPITAL * 100.0;
    let drawdown_pct = max_drawdown * 100.0;

    let equity_rows: Vec<Vec<String>> = equity_curve
        .iter()
        .map(|(ts, cap)| vec![ts.clone(), format!("{:.2}", cap)])
        .collect();

    let safe_name = strategy_name.replace(['/', '\\'], "_");
    let equity_path = format!("logs/equity_{}.csv", safe_name);
    let trades_path = format!("logs/trades_{}.csv", safe_name);

    write_csv(&equity_path, &["timestamp", "capital"], &equity_rows)
        .expect("write equity csv");
    write_csv(
        &trades_path,
        &["trade_id", "timestamp", "side", "price", "pnl", "capital"],
        &trade_rows,
    )
    .expect("write trades csv");

    ResultSummary {
        strategy_name: strategy_name.to_string(),
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
        let position = Some((100.0, 2.0, 200.0));
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

    #[test]
    fn sharpe_ratio_flat_equity_is_zero() {
        let curve = vec![
            ("a".to_string(), 10_000.0),
            ("b".to_string(), 10_000.0),
            ("c".to_string(), 10_000.0),
        ];
        assert_close(sharpe_ratio_from_equity_curve(&curve), 0.0);
    }

    #[test]
    fn sharpe_ratio_two_varying_returns() {
        // returns ≈ 1% and -0.5% → mean ≈ 0.25%, sample std > 0 → finite ratio
        let curve = vec![
            ("a".to_string(), 10_000.0),
            ("b".to_string(), 10_100.0),
            ("c".to_string(), 10_049.5),
        ];
        let s = sharpe_ratio_from_equity_curve(&curve);
        assert!(s.is_finite() && s > 0.0);
    }

    /// One round-trip: open with 10% of cash, close at a higher price; no engine/strategy/IO.
    #[test]
    fn trade_lifecycle_buy_then_sell() {
        let mut cash = 10_000.0;
        let entry_price = 2_000.0;
        let allocation = cash * POSITION_FRACTION;
        let size = allocation / entry_price;
        cash -= allocation;
        let mut position: Option<Position> = Some((entry_price, size, allocation));

        assert_close(calculate_capital(cash, &position, entry_price), 10_000.0);

        let exit_price = 2_200.0;
        let (sz, alloc) = match position.take() {
            Some((ep, s, a)) => {
                assert_close(ep, entry_price);
                (s, a)
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
