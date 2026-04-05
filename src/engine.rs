use crate::models::{Candle, Signal};
use crate::strategy::MovingAverage;

pub fn run(data: Vec<Candle>) {
    let mut strategy = MovingAverage::new(5, 20);

    let mut position: Option<f64> = None;
    let mut total_profit = 0.0;
    let mut trades = 0;

    for candle in &data {
        let signal = strategy.next(candle.close);

        match signal {
            Signal::Buy => {
                if position.is_none() {
                    position = Some(candle.close);
                    println!("BUY at {}", candle.close);
                }
            }
            Signal::Sell => {
                if let Some(entry_price) = position {
                    let profit = candle.close - entry_price;
                    total_profit += profit;
                    trades += 1;

                    println!("SELL at {} | Profit: {}", candle.close, profit);

                    position = None;
                }
            }
            Signal::Hold => {}
        }
    }

    // Close any open position
    if let Some(entry_price) = position {
        let last_price = data.last().unwrap().close;
        let profit = last_price - entry_price;
        total_profit += profit;

        println!("FINAL SELL at {} | Profit: {}", last_price, profit);
    }

    println!("Total Trades: {}", trades);
    println!("Total Profit: {}", total_profit);
}