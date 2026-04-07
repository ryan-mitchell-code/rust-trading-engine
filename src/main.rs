mod csv;
mod data;
mod engine;
mod metrics;
mod models;
mod strategy;

use engine::{run, ResultSummary};
use strategy::{MovingAverage, RandomStrategy};

fn print_run_report(s: &ResultSummary) {
    println!("{}", s.strategy_name);
    println!("  Wrote {}", s.equity_csv);
    println!("  Wrote {}", s.trades_csv);
    println!("  Total Trades: {}", s.total_trades);
    println!("  Win Rate (%): {:.2}", s.win_rate);
    println!("  Total PnL: {:.2}", s.total_pnl);
    println!("  Average PnL: {:.2}", s.avg_pnl);
    println!("  Peak Equity: {:.2}", s.peak_equity);
    println!("  Max Drawdown (%): {:.2}", s.max_drawdown * 100.0);
    println!("  Final Capital: {:.2}", s.final_capital);
}

fn main() {
    let verbose = std::env::args()
        .skip(1)
        .any(|a| a == "-v" || a == "--verbose");

    let candles = data::load_csv("data/formatted_btc.csv");

    let name_ma = "moving_average_5_20";
    let ma = run(&candles, MovingAverage::new(5, 20), name_ma, verbose);
    print_run_report(&ma);

    println!();
    let name_rnd = "random";
    let rnd = run(&candles, RandomStrategy::new(), name_rnd, verbose);
    print_run_report(&rnd);
}
