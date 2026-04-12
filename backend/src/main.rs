mod api;
mod arena;
mod csv;
mod data;
mod engine;
mod metrics;
mod models;
mod paths;
mod strategy;

use engine::BacktestRun;

fn print_comparison_table(backtests: &[engine::BacktestResult]) {
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
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--serve" || a == "-s") {
        api::serve().await;
        return;
    }

    let verbose = args.iter().any(|a| a == "-v" || a == "--verbose");

    // Uses `outputs/binance_cache_*.json` when present so repeat runs skip the API.
    let candles = data::load_from_binance("BTCUSDT", "1d", 1000)
        .await
        .expect("load_from_binance");

    let export: BacktestRun = arena::run_arena(&candles, verbose, 10, 50);

    print_comparison_table(&export.results);

    let json = serde_json::to_string_pretty(&export).expect("serialize backtests");
    std::fs::write(paths::output_file("results.json"), json).expect("write results.json");
}
