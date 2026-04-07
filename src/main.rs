mod csv;
mod data;
mod engine;
mod metrics;
mod models;
mod strategy;

use engine::{run, ResultSummary};
use models::Candle;
use strategy::{MovingAverage, RandomStrategy};

const MOVING_AVERAGE_NAME: &str = "moving_average_5_20";
const RANDOM_NAME: &str = "random";

fn run_moving_average(candles: &[Candle], verbose: bool) -> ResultSummary {
    run(
        candles,
        MovingAverage::new(5, 20),
        MOVING_AVERAGE_NAME,
        verbose,
    )
}

fn run_random(candles: &[Candle], verbose: bool) -> ResultSummary {
    run(candles, RandomStrategy::new(), RANDOM_NAME, verbose)
}

fn print_comparison_table(results: &[ResultSummary]) {
    println!();
    println!(
        "{:<24} {:>8} {:>10} {:>14} {:>12} {:>12} {:>14} {:>12} {:>12}",
        "Strategy",
        "Trades",
        "Sharpe",
        "Final Capital",
        "Total PnL",
        "Avg PnL",
        "Peak Equity",
        "Max DD %",
        "Win Rate %"
    );
    println!("{:-<128}", "");
    for s in results {
        println!(
            "{:<24} {:>8} {:>10.4} {:>14.2} {:>12.2} {:>12.2} {:>14.2} {:>12.2} {:>12.2}",
            s.strategy_name,
            s.total_trades,
            s.sharpe_ratio,
            s.final_capital,
            s.total_pnl,
            s.avg_pnl,
            s.peak_equity,
            s.max_drawdown * 100.0,
            s.win_rate
        );
    }
}

fn main() {
    let verbose = std::env::args()
        .skip(1)
        .any(|a| a == "-v" || a == "--verbose");

    let candles = data::load_csv("data/formatted_btc.csv");

    let strategies: Vec<fn(&[Candle], bool) -> ResultSummary> =
        vec![run_moving_average, run_random];

    let mut results: Vec<ResultSummary> = strategies
        .into_iter()
        .map(|run_strategy| run_strategy(&candles, verbose))
        .collect();

    results.sort_by(|a, b| b.final_capital.total_cmp(&a.final_capital));

    print_comparison_table(&results);
}
