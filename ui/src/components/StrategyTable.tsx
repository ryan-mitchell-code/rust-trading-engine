import type { BacktestResult } from "../types";

type StrategyTableProps = {
  results: BacktestResult[];
  selectedStrategy: string | null;
  onSelectStrategy: (strategyName: string) => void;
};

function fmtPct(n: number, digits = 2): string {
  return `${n.toFixed(digits)}%`;
}

function fmtNum(n: number, digits = 2): string {
  return n.toFixed(digits);
}

export function StrategyTable({
  results,
  selectedStrategy,
  onSelectStrategy,
}: StrategyTableProps) {
  if (results.length === 0) {
    return (
      <p className="text-sm text-slate-500">No strategies to show.</p>
    );
  }

  return (
    <div className="overflow-x-auto rounded-lg border border-slate-800">
      <table className="min-w-full text-left text-sm text-slate-300">
        <thead className="bg-slate-900/80 text-xs uppercase tracking-wide text-slate-500">
          <tr>
            <th className="px-4 py-3 font-medium">Strategy</th>
            <th className="px-4 py-3 font-medium">Return</th>
            <th className="px-4 py-3 font-medium">Drawdown</th>
            <th className="px-4 py-3 font-medium">Sharpe</th>
            <th className="px-4 py-3 font-medium">Score</th>
            <th className="px-4 py-3 font-medium">Trades</th>
            <th className="px-4 py-3 font-medium">Win rate</th>
            <th className="px-4 py-3 font-medium">Avg PnL</th>
            <th className="px-4 py-3 font-medium">Peak equity</th>
            <th className="px-4 py-3 font-medium">DD bars</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-slate-800">
          {results.map((r) => {
            const isSelected = selectedStrategy === r.name;
            return (
            <tr
              key={r.name}
              role="button"
              tabIndex={0}
              onClick={() => {
                onSelectStrategy(r.name);
              }}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  e.preventDefault();
                  onSelectStrategy(r.name);
                }
              }}
              className={`cursor-pointer outline-none transition-colors hover:bg-slate-900/50 focus-visible:ring-2 focus-visible:ring-sky-500/60 ${
                isSelected
                  ? "bg-slate-800/80 ring-1 ring-inset ring-sky-500/40"
                  : ""
              }`}
            >
              <td className="whitespace-nowrap px-4 py-3 font-medium text-slate-100">
                {r.name}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {fmtPct(r.summary.return_pct)}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {fmtPct(r.summary.drawdown_pct)}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {fmtNum(r.summary.sharpe_ratio, 4)}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {fmtNum(r.summary.score, 2)}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {r.summary.total_trades}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {fmtPct(r.summary.win_rate)}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {fmtNum(r.summary.avg_pnl, 2)}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {fmtNum(r.summary.peak_equity, 2)}
              </td>
              <td className="px-4 py-3 tabular-nums">
                {r.summary.max_drawdown_duration}
              </td>
            </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
