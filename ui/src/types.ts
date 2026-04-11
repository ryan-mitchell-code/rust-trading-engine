/**
 * Mirrors `backend/src/engine.rs` (`ResultSummary`, `BacktestResult`) as serialized by `serde_json`.
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

/** Mirrors `EquityPoint` from `backend/src/engine.rs`. */
export interface EquityPoint {
  timestamp: string;
  capital: number;
}

/** Mirrors `DrawdownPoint` from `backend/src/engine.rs`. */
export interface DrawdownPoint {
  timestamp: string;
  /** `(equity - peak) / peak`; ≤ 0 vs running peak. */
  drawdown: number;
}

export interface BacktestResult {
  /** Strategy id — stable key for this backtest (not duplicated under `summary`). */
  name: string;
  summary: ResultSummary;
  equity_curve: EquityPoint[];
  drawdown_curve: DrawdownPoint[];
  trades: string[][];
}
