/**
 * Mirrors `backend/src/engine.rs` (`ResultSummary`, `BacktestResult`) as serialized by `serde_json`.
 * If the UI drifts from real output, fix the Rust types / serialization — not ad-hoc mapping here.
 */

export interface ResultSummary {
  strategy_name: string;
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

/** `Vec<(String, f64)>` → JSON array of `[timestamp, capital]` pairs. */
export type EquityCurve = [string, number][];

export interface BacktestResult {
  /** Strategy id (mirrors `summary.strategy_name` from backend). */
  name: string;
  summary: ResultSummary;
  equity_curve: EquityCurve;
  trades: string[][];
}
