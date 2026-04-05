use crate::csv::write_csv;
use crate::models::{Candle, Signal};
use crate::strategy::MovingAverage;

const INITIAL_CAPITAL: f64 = 10_000.0;
const POSITION_FRACTION: f64 = 0.10;

/// Open position: `(entry_price per unit, size in units, cash allocated at entry)`.
type Position = (f64, f64, f64);

pub fn run(data: Vec<Candle>) {
    let mut strategy = MovingAverage::new(5, 20);

    let mut cash = INITIAL_CAPITAL;
    let mut position: Option<Position> = None;
    let mut open_trade_id: Option<u32> = None;
    let mut next_trade_id: u32 = 1;
    let mut trades = 0;
    let mut trade_rows: Vec<Vec<String>> = Vec::new();
    let mut equity_curve: Vec<(String, f64)> = Vec::with_capacity(data.len());

    for candle in &data {
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

                        let capital = cash + size * candle.close;

                        println!(
                            "{} | id {} | BUY | price {:.6} | pnl 0.00 | capital {:.2}",
                            candle.timestamp, trade_id, candle.close, capital
                        );

                        trade_rows.push(vec![
                            trade_id.to_string(),
                            candle.timestamp.clone(),
                            "BUY".to_string(),
                            format!("{:.6}", candle.close),
                            format!("{:.2}", 0.0_f64),
                            format!("{:.2}", capital),
                        ]);
                    }
                }
            }
            Signal::Sell => {
                if let Some((_, size, allocation)) = position.take() {
                    let exit_price = candle.close;
                    let proceeds = size * exit_price;
                    let pnl = proceeds - allocation;

                    cash += proceeds;
                    trades += 1;

                    let trade_id = open_trade_id
                        .take()
                        .expect("sell should follow a logged buy");
                    let capital = cash;

                    println!(
                        "{} | id {} | SELL | price {:.6} | pnl {:.2} | capital {:.2}",
                        candle.timestamp, trade_id, exit_price, pnl, capital
                    );

                    trade_rows.push(vec![
                        trade_id.to_string(),
                        candle.timestamp.clone(),
                        "SELL".to_string(),
                        format!("{:.6}", exit_price),
                        format!("{:.2}", pnl),
                        format!("{:.2}", capital),
                    ]);
                }
            }
            Signal::Hold => {}
        }

        let held = position.map(|(_, size, _)| size).unwrap_or(0.0);
        let capital = cash + held * candle.close;
        equity_curve.push((candle.timestamp.clone(), capital));
    }

    if let Some((_, size, allocation)) = position.take() {
        let last = data.last().expect("data not empty");
        let exit_price = last.close;
        let proceeds = size * exit_price;
        let pnl = proceeds - allocation;

        cash += proceeds;
        trades += 1;

        let trade_id = open_trade_id
            .take()
            .expect("final sell should follow a logged buy");
        let capital = cash;

        println!(
            "{} | id {} | SELL (final) | price {:.6} | pnl {:.2} | capital {:.2}",
            last.timestamp, trade_id, exit_price, pnl, capital
        );

        trade_rows.push(vec![
            trade_id.to_string(),
            last.timestamp.clone(),
            "SELL".to_string(),
            format!("{:.6}", exit_price),
            format!("{:.2}", pnl),
            format!("{:.2}", capital),
        ]);

        if let Some(last_row) = equity_curve.last_mut() {
            *last_row = (last.timestamp.clone(), capital);
        }
    }

    println!("Total Trades: {}", trades);
    println!("Final Capital: {:.2}", cash);

    let equity_rows: Vec<Vec<String>> = equity_curve
        .iter()
        .map(|(ts, cap)| vec![ts.clone(), format!("{:.2}", cap)])
        .collect();

    write_csv("logs/equity.csv", &["timestamp", "capital"], &equity_rows)
        .expect("write logs/equity.csv");
    write_csv(
        "logs/trades.csv",
        &["trade_id", "timestamp", "side", "price", "pnl", "capital"],
        &trade_rows,
    )
    .expect("write logs/trades.csv");

    println!("Wrote logs/equity.csv");
    println!("Wrote logs/trades.csv");
}
