mod csv;
mod data;
mod engine;
mod metrics;
mod models;
mod strategy;

use engine::{run, ResultSummary};
use models::Candle;
use strategy::{BuyAndHold, MovingAverage, RandomStrategy};

const MOVING_AVERAGE_NAME: &str = "moving_average_5_20";
const RANDOM_NAME: &str = "random";
const BUY_AND_HOLD_NAME: &str = "buy_and_hold";

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

fn run_buy_and_hold(candles: &[Candle], verbose: bool) -> ResultSummary {
    run(candles, BuyAndHold::new(), BUY_AND_HOLD_NAME, verbose)
}

fn print_comparison_table(results: &[ResultSummary]) {
    println!();
    println!(
        "{:<24} {:>10} {:>10} {:>10} {:>10} {:>8} {:>14} {:>12} {:>12} {:>14} {:>10} {:>12}",
        "Strategy",
        "Return %",
        "Drawdown %",
        "Sharpe",
        "Score",
        "Trades",
        "Final Capital",
        "Total PnL",
        "Avg PnL",
        "Peak Equity",
        "DD Bars",
        "Win Rate %"
    );
    println!("{:-<153}", "");
    for s in results {
        println!(
            "{:<24} {:>10.2} {:>10.2} {:>10.4} {:>10.2} {:>8} {:>14.2} {:>12.2} {:>12.2} {:>14.2} {:>10} {:>12.2}",
            s.strategy_name,
            s.return_pct,
            s.drawdown_pct,
            s.sharpe_ratio,
            s.score,
            s.total_trades,
            s.final_capital,
            s.total_pnl,
            s.avg_pnl,
            s.peak_equity,
            s.max_drawdown_duration,
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
        vec![run_moving_average, run_random, run_buy_and_hold];

    let mut results: Vec<ResultSummary> = strategies
        .into_iter()
        .map(|run_strategy| run_strategy(&candles, verbose))
        .collect();

    results.sort_by(|a, b| b.score.total_cmp(&a.score));

    print_comparison_table(&results);
}
