import { cardClass } from "../constants/layout.ts";
import { cn } from "../lib/cn.ts";
import {
  metricDrawdownToneClass,
  metricRelativeVsBhToneClass,
  metricReturnToneClass,
  metricSharpeToneClass,
} from "../lib/metricStyles.ts";
import type { BacktestResult } from "../types.ts";
import { HelpHint } from "./HelpHint.tsx";

type StrategyMetricCardsProps = {
  results: BacktestResult[];
  selectedStrategy: string | null;
  bestStrategyName: string | null;
  onToggleStrategy: (strategyName: string) => void;
};

export function StrategyMetricCards({
  results,
  selectedStrategy,
  bestStrategyName,
  onToggleStrategy,
}: StrategyMetricCardsProps) {
  return (
    <section
      className={`space-y-5 ${cardClass}`}
      aria-label="Strategy metrics"
    >
      <div className="flex flex-col gap-3 border-b border-slate-800/90 pb-4 sm:flex-row sm:items-end sm:justify-between sm:gap-4">
        <div className="min-w-0">
          <h2 className="text-sm font-semibold tracking-tight text-slate-200">
            Strategies
          </h2>
          <p className="mt-1 text-xs leading-relaxed text-slate-500">
            Per-run snapshot: return, drawdown, Sharpe, and vs buy-and-hold. Your
            choice here focuses the charts and markers below.
          </p>
        </div>
        <HelpHint
          label="How strategy selection works"
          text="Click a strategy card to focus charts and trade markers on that run. Click the same card again to show all strategies."
        />
      </div>

      <div
        className="flex flex-wrap gap-3"
        role="list"
        aria-label="Strategy metric cards"
      >
        {results.map((r) => {
          const isSelected = selectedStrategy === r.name;
          const isBest =
            bestStrategyName !== null && r.name === bestStrategyName;
          const s = r.summary;
          const vsBh = s.relative_return;
          return (
            <button
              key={r.name}
              type="button"
              role="listitem"
              aria-pressed={isSelected}
              onClick={() => onToggleStrategy(r.name)}
              className={cn(
                "w-full min-w-[10rem] max-w-full flex-1 rounded-lg border px-4 py-3 text-left transition-all duration-150 hover:-translate-y-[1px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-500/60 sm:max-w-[15rem]",
                isSelected
                  ? "border-sky-500 bg-slate-800/95 shadow-sm ring-1 ring-sky-500/35"
                  : isBest
                    ? "border-emerald-500/40 bg-emerald-500/5 hover:border-emerald-500/50 hover:bg-emerald-500/[0.08]"
                    : "border-slate-800 bg-slate-900/45 hover:border-slate-600 hover:bg-slate-900/75",
              )}
            >
              <div
                className="truncate text-sm font-semibold tracking-tight text-slate-100"
                title={r.name}
              >
                {r.name}
              </div>
              <dl className="mt-2 space-y-1 text-xs">
                <div className="flex justify-between gap-3">
                  <dt className="text-slate-500">Return</dt>
                  <dd
                    className={cn(
                      "tabular-nums",
                      metricReturnToneClass(s.return_pct),
                    )}
                  >
                    {s.return_pct.toFixed(2)}%
                  </dd>
                </div>
                <div className="flex justify-between gap-3">
                  <dt className="text-slate-500">Drawdown</dt>
                  <dd
                    className={cn(
                      "tabular-nums",
                      metricDrawdownToneClass(),
                    )}
                  >
                    {s.drawdown_pct.toFixed(2)}%
                  </dd>
                </div>
                <div className="flex justify-between gap-3">
                  <dt className="text-slate-500">Sharpe</dt>
                  <dd
                    className={cn(
                      "tabular-nums",
                      metricSharpeToneClass(),
                    )}
                  >
                    {s.sharpe_ratio.toFixed(4)}
                  </dd>
                </div>
                <div className="flex justify-between gap-3">
                  <dt className="text-slate-500">vs B&H</dt>
                  <dd
                    className={cn(
                      "tabular-nums",
                      metricRelativeVsBhToneClass(vsBh),
                    )}
                  >
                    {vsBh >= 0 ? "+" : ""}
                    {vsBh.toFixed(2)}%
                  </dd>
                </div>
              </dl>
            </button>
          );
        })}
      </div>
    </section>
  );
}
