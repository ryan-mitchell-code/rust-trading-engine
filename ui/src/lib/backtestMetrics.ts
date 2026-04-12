import type { BacktestResult } from "../types.ts";

export function findSelectedResult(
  results: BacktestResult[],
  selectedStrategy: string | null,
): BacktestResult | undefined {
  if (selectedStrategy === null) return undefined;
  return results.find((r) => r.name === selectedStrategy);
}

/** Strategy with the highest `return_pct` (first wins on ties). */
export function bestStrategyNameByReturn(
  results: BacktestResult[],
): string | null {
  if (results.length === 0) return null;
  let maxReturn = -Infinity;
  let name: string | null = null;
  for (const r of results) {
    if (r.summary.return_pct > maxReturn) {
      maxReturn = r.summary.return_pct;
      name = r.name;
    }
  }
  return name;
}
