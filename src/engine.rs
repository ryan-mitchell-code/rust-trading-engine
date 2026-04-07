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

pub fn run<S: Strategy>(data: &[Candle], mut strategy: S, strategy_name: &str) {
    println!("=== {} ===", strategy_name);

    let mut cash = INITIAL_CAPITAL;
    let mut position: Option<Position> = None;
    let mut open_trade_id: Option<u32> = None;
    let mut next_trade_id: u32 = 1;
    let mut metrics = Metrics::new(INITIAL_CAPITAL);
    let mut trade_rows: Vec<Vec<String>> = Vec::new();
    let mut equity_curve: Vec<(String, f64)> = Vec::with_capacity(data.len());

    for candle in data {
        let signal = strategy.next(candle.close);

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

                        let capital = calculate_capital(cash, &position, candle.close);

                        println!(
                            "{} | id {} | BUY | price {:.6} | pnl 0.00 | capital {:.2}",
                            candle.timestamp, trade_id, candle.close, capital
                        );

                        trade_rows.push(make_trade_row(
                            trade_id,
                            &candle.timestamp,
                            "BUY",
                            candle.close,
                            0.0_f64,
                            capital,
                        ));
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
                    let capital = calculate_capital(cash, &position, exit_price);

                    println!(
                        "{} | id {} | SELL | price {:.6} | pnl {:.2} | capital {:.2}",
                        candle.timestamp, trade_id, exit_price, pnl, capital
                    );

                    trade_rows.push(make_trade_row(
                        trade_id,
                        &candle.timestamp,
                        "SELL",
                        exit_price,
                        pnl,
                        capital,
                    ));
                }
            }
            Signal::Hold => {}
        }

        let capital = calculate_capital(cash, &position, candle.close);
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

        println!(
            "{} | id {} | SELL (final) | price {:.6} | pnl {:.2} | capital {:.2}",
            last.timestamp, trade_id, exit_price, pnl, capital
        );

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

    println!("Total Trades: {}", metrics.trades());
    println!("Win Rate (%): {:.2}", metrics.win_rate());
    println!("Total PnL: {:.2}", metrics.total_pnl());
    println!("Average PnL: {:.2}", metrics.avg_pnl());
    println!("Peak Equity: {:.2}", metrics.peak_equity());
    println!("Max Drawdown (%): {:.2}", metrics.max_drawdown() * 100.0);
    println!("Final Capital: {:.2}", cash);

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

    println!("Wrote {}", equity_path);
    println!("Wrote {}", trades_path);
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
