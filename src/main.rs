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
const SHARPE_WEIGHT: f64 = 2.0;
const RETURN_WEIGHT: f64 = 1.0;
const DRAWDOWN_WEIGHT: f64 = 1.0;

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

fn apply_scoring(results: &mut [ResultSummary]) {
    let max_abs_sharpe = results
        .iter()
        .map(|r| r.sharpe_ratio.abs())
        .fold(0.0_f64, f64::max);
    let max_abs_return = results
        .iter()
        .map(|r| r.return_pct.abs())
        .fold(0.0_f64, f64::max);
    let max_abs_drawdown = results
        .iter()
        .map(|r| r.drawdown_pct.abs())
        .fold(0.0_f64, f64::max);

    for r in results.iter_mut() {
        let normalized_sharpe = if max_abs_sharpe > f64::EPSILON {
            r.sharpe_ratio / max_abs_sharpe
        } else {
            0.0
        };
        let normalized_return = if max_abs_return > f64::EPSILON {
            r.return_pct / max_abs_return
        } else {
            0.0
        };
        let normalized_drawdown = if max_abs_drawdown > f64::EPSILON {
            r.drawdown_pct / max_abs_drawdown
        } else {
            0.0
        };

        r.score_sharpe_component = normalized_sharpe * SHARPE_WEIGHT;
        r.score_return_component = normalized_return * RETURN_WEIGHT;
        r.score_drawdown_component = normalized_drawdown * DRAWDOWN_WEIGHT;
        r.score =
            r.score_sharpe_component + r.score_return_component - r.score_drawdown_component;
    }
}

fn print_comparison_table(results: &[ResultSummary]) {
    println!();
    println!(
        "{:<24} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12} {:>12} {:>10} {:>8} {:>14} {:>12} {:>12} {:>14} {:>10} {:>12}",
        "Strategy",
        "Return %",
        "vs B&H %",
        "Drawdown %",
        "Sharpe",
        "Sharpe Contr",
        "Return Contr",
        "DD Penalty",
        "Score",
        "Trades",
        "Final Capital",
        "Total PnL",
        "Avg PnL",
        "Peak Equity",
        "DD Bars",
        "Win Rate %"
    );
    println!("{:-<201}", "");
    for s in results {
        println!(
            "{:<24} {:>10.2} {:>10.2} {:>10.2} {:>10.4} {:>12.2} {:>12.2} {:>12.2} {:>10.2} {:>8} {:>14.2} {:>12.2} {:>12.2} {:>14.2} {:>10} {:>12.2}",
            s.strategy_name,
            s.return_pct,
            s.relative_return,
            s.drawdown_pct,
            s.sharpe_ratio,
            s.score_sharpe_component,
            s.score_return_component,
            s.score_drawdown_component,
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

    let bh_return = results
        .iter()
        .find(|r| r.strategy_name == BUY_AND_HOLD_NAME)
        .expect("buy_and_hold in results")
        .return_pct;
    for r in &mut results {
        r.relative_return = r.return_pct - bh_return;
    }

    apply_scoring(&mut results);

    results.sort_by(|a, b| b.score.total_cmp(&a.score));

    print_comparison_table(&results);
}
