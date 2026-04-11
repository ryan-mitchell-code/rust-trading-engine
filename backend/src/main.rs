mod csv;
mod data;
mod engine;
mod metrics;
mod models;
mod paths;
mod strategy;

use engine::{market_series, run, BacktestResult, BacktestRun};
use strategy::{BuyAndHold, MovingAverage, RandomStrategy};

const MOVING_AVERAGE_NAME: &str = "moving_average_5_20";
const RANDOM_NAME: &str = "random";
const BUY_AND_HOLD_NAME: &str = "buy_and_hold";
const SHARPE_WEIGHT: f64 = 2.0;
const RETURN_WEIGHT: f64 = 1.0;
const DRAWDOWN_WEIGHT: f64 = 1.0;

fn write_backtest_outputs(market: &[(String, f64)], bt: &BacktestResult) {
    let equity_rows: Vec<Vec<String>> = market
        .iter()
        .zip(bt.equity_curve.iter())
        .map(|((ts, _), cap)| vec![ts.clone(), format!("{:.2}", cap)])
        .collect();
    csv::write_csv(&bt.summary.equity_csv, &["timestamp", "capital"], &equity_rows)
        .expect("write equity csv");
    csv::write_csv(
        &bt.summary.trades_csv,
        &["trade_id", "timestamp", "side", "price", "pnl", "capital"],
        &bt.trades,
    )
    .expect("write trades csv");
}

fn apply_scoring(backtests: &mut [BacktestResult]) {
    let max_abs_sharpe = backtests
        .iter()
        .map(|b| b.summary.sharpe_ratio.abs())
        .fold(0.0_f64, f64::max);
    let max_abs_return = backtests
        .iter()
        .map(|b| b.summary.return_pct.abs())
        .fold(0.0_f64, f64::max);
    let max_abs_drawdown = backtests
        .iter()
        .map(|b| b.summary.drawdown_pct.abs())
        .fold(0.0_f64, f64::max);

    for b in backtests.iter_mut() {
        let r = &mut b.summary;
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

fn print_comparison_table(backtests: &[BacktestResult]) {
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
    for b in backtests {
        let s = &b.summary;
        println!(
            "{:<24} {:>10.2} {:>10.2} {:>10.2} {:>10.4} {:>12.2} {:>12.2} {:>12.2} {:>10.2} {:>8} {:>14.2} {:>12.2} {:>12.2} {:>14.2} {:>10} {:>12.2}",
            b.name,
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

#[tokio::main]
async fn main() {
    let verbose = std::env::args()
        .skip(1)
        .any(|a| a == "-v" || a == "--verbose");

    // Uses `outputs/binance_cache_*.json` when present so repeat runs skip the API.
    let candles = data::load_from_binance("BTCUSDT", "1d", 1000).await;
    let market = market_series(&candles);

    let mut backtests: Vec<BacktestResult> = vec![
        run(
            &candles,
            MovingAverage::new(5, 20),
            MOVING_AVERAGE_NAME,
            verbose,
        ),
        run(&candles, RandomStrategy::new(), RANDOM_NAME, verbose),
        run(&candles, BuyAndHold::new(), BUY_AND_HOLD_NAME, verbose),
    ];

    for bt in &backtests {
        write_backtest_outputs(&market, bt);
    }

    let bh_return = backtests
        .iter()
        .find(|b| b.name == BUY_AND_HOLD_NAME)
        .expect("buy_and_hold in backtests")
        .summary
        .return_pct;
    for bt in &mut backtests {
        bt.summary.relative_return = bt.summary.return_pct - bh_return;
    }

    apply_scoring(&mut backtests);

    backtests.sort_by(|a, b| b.summary.score.total_cmp(&a.summary.score));

    print_comparison_table(&backtests);

    let export = BacktestRun {
        market,
        results: backtests,
    };
    let json = serde_json::to_string_pretty(&export).expect("serialize backtests");
    std::fs::write(paths::output_file("results.json"), json).expect("write results.json");
}
