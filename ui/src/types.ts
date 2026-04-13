/**
 * Mirrors `backend/src/engine` (`BacktestResult` / `BacktestRun`) as serialized by `serde_json`.
 * If the UI drifts from real output, fix the Rust types / serialization — not ad-hoc mapping here.
 */

export interface ResultSummary {
  equity_csv: string;
  trades_csv: string;
  final_capital: number;
  total_pnl: number;
  total_trades: number;
  win_rate: number;
  avg_pnl: number;
  peak_equity: number;
  max_drawdown: number;
  max_drawdown_duration: number;
  return_pct: number;
  relative_return: number;
  drawdown_pct: number;
  sharpe_ratio: number;
  score_sharpe_component: number;
  score_return_component: number;
  score_drawdown_component: number;
  score: number;
}

/** `Vec<(String, f64, f64, f64, f64)>` → JSON array of `[timestamp, open, high, low, close]` per bar. */
export type MarketSeries = [string, number, number, number, number][];

export interface BacktestResult {
  /** Strategy id — stable key for this backtest (not duplicated under `summary`). */
  name: string;
  summary: ResultSummary;
  /** Mark-to-market capital per bar; same length as `BacktestRun.market`. */
  equity_curve: number[];
  /** Drawdown ratio vs running peak per bar; same length as `equity_curve`. */
  drawdown_curve: number[];
  trades: string[][];
}

/** `BacktestRun` from backend — shared market + per-strategy curves (no duplicated timestamps in results). */
export interface BacktestRun {
  market: MarketSeries;
  results: BacktestResult[];
}
