/** Shared Recharts series styling — equity + drawdown charts stay visually aligned. */

export const SERIES_COLORS = [
  "#38bdf8",
  "#818cf8",
  "#34d399",
  "#fbbf24",
  "#f472b6",
] as const;

export const LINE_STROKE_WIDTH = 2;
export const LINE_STROKE_WIDTH_HIGHLIGHT = 3;

/** Matches `BUY_AND_HOLD_NAME` in backend — passive benchmark, not an active strategy. */
export const BENCHMARK_STRATEGY_NAME = "buy_and_hold";
export const BENCHMARK_STROKE = "#94a3b8";
/** Thin stroke so dash pattern reads as dotted vs solid active series. */
export const BENCHMARK_STROKE_WIDTH = 1.15;
/** Short gaps + small dashes → visibly dotted (with round line caps). */
export const BENCHMARK_STROKE_DASHARRAY = "1.5 5";
