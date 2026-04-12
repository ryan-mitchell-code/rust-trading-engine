//! Multi-strategy backtest orchestration: run engines, write CSVs, benchmark-relative return, scoring, ranking.

use crate::csv;
use crate::engine::{market_series, run, BacktestResult, BacktestRun};
use crate::models::Candle;
use crate::strategy::{BuyAndHold, MovingAverage, RandomStrategy, RsiStrategy};

const RSI_NAME: &str = "RSI";
const RANDOM_NAME: &str = "random";
const BUY_AND_HOLD_NAME: &str = "buy_and_hold";

const SHARPE_WEIGHT: f64 = 2.0;
const RETURN_WEIGHT: f64 = 1.0;
const DRAWDOWN_WEIGHT: f64 = 1.0;

fn write_backtest_outputs(market: &[(String, f64, f64, f64, f64)], bt: &BacktestResult) {
    let equity_rows: Vec<Vec<String>> = market
        .iter()
        .zip(bt.equity_curve.iter())
        .map(|((ts, _, _, _, _), cap)| vec![ts.clone(), format!("{:.2}", cap)])
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

/// Run the default strategy set on `candles`, write per-strategy CSVs, apply vs–buy-and-hold deltas and arena scoring, then sort by score (highest first).
pub fn run_arena(candles: &[Candle], verbose: bool, ma_short: usize, ma_long: usize) -> BacktestRun {
    let market = market_series(candles);

    let moving_average_name = format!("moving_average_{ma_short}_{ma_long}");
    let mut backtests: Vec<BacktestResult> = vec![
        run(
            candles,
            MovingAverage::new(ma_short, ma_long),
            &moving_average_name,
            verbose,
        ),
        run(
            candles,
            RsiStrategy::new(14, 70.0, 30.0),
            RSI_NAME,
            verbose,
        ),
        run(candles, RandomStrategy::new(), RANDOM_NAME, verbose),
        run(candles, BuyAndHold::new(), BUY_AND_HOLD_NAME, verbose),
    ];

    for bt in &backtests {
        write_backtest_outputs(&market, bt);
    }

    let bh_return = backtests
        .iter()
        .find(|b| b.name == BUY_AND_HOLD_NAME)
        .map(|b| b.summary.return_pct);
    for bt in &mut backtests {
        bt.summary.relative_return = match bh_return {
            Some(r) => bt.summary.return_pct - r,
            None => 0.0,
        };
    }

    apply_scoring(&mut backtests);

    backtests.sort_by(|a, b| b.summary.score.total_cmp(&a.summary.score));

    BacktestRun {
        market,
        results: backtests,
    }
}
