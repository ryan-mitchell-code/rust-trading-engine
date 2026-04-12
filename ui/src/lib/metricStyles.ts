/**
 * Shared Tailwind color classes for strategy metrics (cards + table).
 * Aligns with dashboard semantics: return / vs B&H by sign, drawdown always red, Sharpe sky.
 */

export function metricReturnToneClass(returnPct: number): string {
  return returnPct >= 0 ? "text-emerald-400" : "text-red-400";
}

export function metricDrawdownToneClass(): string {
  return "text-red-400";
}

export function metricSharpeToneClass(): string {
  return "text-sky-400";
}

/** Relative return vs buy & hold (same sign rule as `metricReturnToneClass`). */
export function metricRelativeVsBhToneClass(relativeReturn: number): string {
  return relativeReturn >= 0 ? "text-emerald-400" : "text-red-400";
}
