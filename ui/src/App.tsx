import { useCallback, useId, useState } from "react";
import { DrawdownChart } from "./components/DrawdownChart.tsx";
import { EquityChart } from "./components/EquityChart.tsx";
import { PriceChart } from "./components/PriceChart.tsx";
import { StrategyTable } from "./components/StrategyTable.tsx";
import { fetchBacktestRun, type Dataset } from "./services/api.ts";
import type { BacktestResult, BacktestRun } from "./types.ts";

function findSelectedResult(
  results: BacktestResult[],
  selectedStrategy: string | null,
): BacktestResult | undefined {
  if (selectedStrategy === null) return undefined;
  return results.find((r) => r.name === selectedStrategy);
}

const shellMaxClass = "mx-auto w-full max-w-[1600px]";

const cardClass =
  "rounded-xl border border-slate-800 bg-slate-900/60 px-5 py-5 shadow-sm sm:px-6 sm:py-6";

type HelpHintProps = {
  /** Shown in the tooltip panel */
  text: string;
  /** Accessible name for the help control */
  label: string;
};

function HelpHint({ text, label }: HelpHintProps) {
  const tipId = useId();
  return (
    <span className="group relative inline-flex shrink-0">
      <button
        type="button"
        className="inline-flex h-5 min-w-5 items-center justify-center rounded-full border border-slate-600 bg-slate-800/90 px-1 text-[10px] font-semibold leading-none text-slate-400 transition hover:border-slate-500 hover:text-slate-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-500/60"
        aria-label={label}
        aria-describedby={tipId}
      >
        ?
      </button>
      <span
        id={tipId}
        role="tooltip"
        className="pointer-events-none invisible absolute left-1/2 top-full z-40 mt-2 w-64 max-w-[min(18rem,calc(100vw-2rem))] -translate-x-1/2 rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-left text-xs leading-relaxed text-slate-300 opacity-0 shadow-xl transition-opacity duration-150 group-hover:visible group-hover:opacity-100 group-focus-within:visible group-focus-within:opacity-100"
      >
        {text}
      </span>
    </span>
  );
}

type DashboardView = "charts" | "table";

export function App() {
  const [run, setRun] = useState<BacktestRun | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [dataset, setDataset] = useState<Dataset>("BTCUSDT");
  const [view, setView] = useState<DashboardView>("charts");
  const [selectedStrategy, setSelectedStrategy] = useState<string | null>(null);

  const handleToggleStrategy = useCallback((strategyName: string) => {
    setSelectedStrategy((prev) =>
      prev === strategyName ? null : strategyName,
    );
  }, []);

  const handleRunBacktest = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await fetchBacktestRun(dataset);
      setRun(data);
    } catch (e: unknown) {
      const message =
        e instanceof Error ? e.message : "Failed to run backtest";
      setError(message);
      setRun(null);
    } finally {
      setLoading(false);
      setSelectedStrategy(null);
    }
  }, [dataset]);

  const selected =
    run && findSelectedResult(run.results, selectedStrategy);
  const tradesForChart = selected?.trades;

  let bestStrategyName: string | null = null;
  if (run !== null) {
    let maxReturn = -Infinity;
    for (const r of run.results) {
      if (r.summary.return_pct > maxReturn) {
        maxReturn = r.summary.return_pct;
        bestStrategyName = r.name;
      }
    }
  }

  return (
    <div className="min-h-dvh bg-slate-950 text-slate-100">
      <header className="sticky top-0 z-20 border-b border-slate-800/90 bg-slate-950/75 px-4 py-3 backdrop-blur-md lg:px-6 xl:px-8 sm:py-4">
        <div
          className={`flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between ${shellMaxClass}`}
        >
          <div className="min-w-0">
            <h1 className="text-lg font-semibold tracking-tight text-slate-50 sm:text-xl">
              Rust Trader
            </h1>
            <p className="mt-0.5 text-xs text-slate-500 sm:text-sm">
              Backtest overview —{" "}
              <code className="text-slate-400">POST /run</code>
            </p>
          </div>
          <div className="flex shrink-0 flex-wrap items-center gap-3 rounded-xl border border-slate-800 bg-slate-900/60 px-3 py-2.5 shadow-sm sm:gap-4 sm:px-4 sm:py-3">
            <label
              htmlFor="dataset-select"
              className="text-sm font-medium text-slate-400"
            >
              Dataset
            </label>
            <select
              id="dataset-select"
              value={dataset}
              onChange={(e) => setDataset(e.target.value as Dataset)}
              disabled={loading}
              className="select-inset-chevron rounded-lg border border-slate-700 bg-slate-950/80 py-2 pl-3 pr-10 text-sm text-slate-100 shadow-sm outline-none ring-sky-500/30 transition focus:border-sky-600 focus:ring-2 disabled:cursor-not-allowed disabled:opacity-50"
            >
              <option value="BTCUSDT">BTCUSDT</option>
              <option value="ETHUSDT">ETHUSDT</option>
            </select>
            <button
              type="button"
              onClick={handleRunBacktest}
              disabled={loading}
              className="rounded-lg bg-sky-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-sky-500 disabled:cursor-not-allowed disabled:opacity-50"
            >
              {loading ? "Running…" : "Run Backtest"}
            </button>
          </div>
        </div>
      </header>

      <main
        className={`space-y-8 px-4 py-6 sm:py-8 lg:px-6 lg:py-10 xl:px-8 ${shellMaxClass}`}
      >
        {error !== null && (
          <div
            className="rounded-xl border border-amber-900/50 bg-amber-950/35 px-5 py-4 text-sm text-amber-200 shadow-sm sm:px-6 sm:py-5"
            role="alert"
          >
            Could not load backtests: {error}
          </div>
        )}

        {loading && (
          <div className="rounded-xl border border-slate-800 bg-slate-900/60 px-5 py-5 shadow-sm sm:px-6 sm:py-6">
            <p className="animate-pulse text-sm text-slate-400">
              Running backtest…
            </p>
          </div>
        )}

        {!loading && run === null && error === null && (
          <div className="rounded-xl border border-slate-800 bg-slate-900/60 px-5 py-10 text-center shadow-sm sm:px-6 sm:py-12">
            <p className="text-sm text-slate-500">
              Click{" "}
              <span className="font-medium text-slate-400">Run Backtest</span>{" "}
              to load results.
            </p>
          </div>
        )}

        {run !== null && (
          <div className="space-y-8">
            <div className={`space-y-4 ${cardClass}`}>
              <div className="flex justify-end">
                <HelpHint
                  label="How strategy selection works"
                  text="Click a strategy card to focus charts and trade markers on that run. Click the same card again to show all strategies."
                />
              </div>

              <div
                className="flex flex-wrap gap-3"
                role="list"
                aria-label="Strategy metrics"
              >
                {run.results.map((r) => {
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
                      onClick={() => handleToggleStrategy(r.name)}
                      className={`w-full min-w-[10rem] max-w-full flex-1 rounded-lg border px-4 py-3 text-left transition-all duration-150 hover:-translate-y-[1px] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-500/60 sm:max-w-[15rem] ${
                        isSelected
                          ? "border-sky-500 bg-slate-800/95 shadow-sm ring-1 ring-sky-500/35"
                          : isBest
                            ? "border-emerald-500/40 bg-emerald-500/5 hover:border-emerald-500/50 hover:bg-emerald-500/[0.08]"
                            : "border-slate-800 bg-slate-900/45 hover:border-slate-600 hover:bg-slate-900/75"
                      }`}
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
                            className={`tabular-nums ${
                              s.return_pct >= 0
                                ? "text-emerald-400"
                                : "text-red-400"
                            }`}
                          >
                            {s.return_pct.toFixed(2)}%
                          </dd>
                        </div>
                        <div className="flex justify-between gap-3">
                          <dt className="text-slate-500">Drawdown</dt>
                          <dd className="tabular-nums text-red-400">
                            {s.drawdown_pct.toFixed(2)}%
                          </dd>
                        </div>
                        <div className="flex justify-between gap-3">
                          <dt className="text-slate-500">Sharpe</dt>
                          <dd className="tabular-nums text-sky-400">
                            {s.sharpe_ratio.toFixed(4)}
                          </dd>
                        </div>
                        <div className="flex justify-between gap-3">
                          <dt className="text-slate-500">vs B&H</dt>
                          <dd
                            className={`tabular-nums ${
                              vsBh >= 0 ? "text-emerald-400" : "text-red-400"
                            }`}
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
            </div>

            <div
              className="inline-flex rounded-lg border border-slate-800 bg-slate-900/60 p-1 shadow-sm"
              role="tablist"
              aria-label="Dashboard view"
            >
              <button
                type="button"
                role="tab"
                aria-selected={view === "charts"}
                onClick={() => setView("charts")}
                className={`rounded-md px-4 py-2 text-sm font-medium transition ${
                  view === "charts"
                    ? "bg-sky-600 text-white shadow-sm ring-1 ring-sky-400/40"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                Charts
              </button>
              <button
                type="button"
                role="tab"
                aria-selected={view === "table"}
                onClick={() => setView("table")}
                className={`rounded-md px-4 py-2 text-sm font-medium transition ${
                  view === "table"
                    ? "bg-sky-600 text-white shadow-sm ring-1 ring-sky-400/40"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                Table
              </button>
            </div>

            {view === "charts" ? (
              <div className="space-y-8">
                <section className={`space-y-4 ${cardClass}`}>
                  <h2 className="text-sm font-semibold tracking-tight text-slate-200">
                    Market Price — {dataset}
                  </h2>
                  <div className="pt-2">
                    <PriceChart market={run.market} trades={tradesForChart} />
                  </div>
                </section>

                <div className="grid grid-cols-1 gap-8 md:grid-cols-2">
                  <section className={`space-y-4 ${cardClass}`}>
                    <h2 className="text-sm font-semibold tracking-tight text-slate-200">
                      Equity curves — {dataset}
                    </h2>
                    <div className="pt-2">
                      <EquityChart
                        market={run.market}
                        results={run.results}
                        selectedStrategy={selectedStrategy}
                      />
                    </div>
                  </section>

                  <section className={`space-y-4 ${cardClass}`}>
                    <h2 className="text-sm font-semibold tracking-tight text-slate-200">
                      Drawdown — {dataset}
                    </h2>
                    <div className="pt-2">
                      <DrawdownChart
                        market={run.market}
                        results={run.results}
                        selectedStrategy={selectedStrategy}
                      />
                    </div>
                  </section>
                </div>
              </div>
            ) : (
              <section className={`space-y-4 ${cardClass}`}>
                <div className="flex items-start justify-between gap-3">
                  <h2 className="text-sm font-semibold tracking-tight text-slate-200">
                    Strategy comparison
                  </h2>
                  <HelpHint
                    label="How table selection works"
                    text="Click a row to focus charts and trade markers on that strategy; click again to clear."
                  />
                </div>
                <StrategyTable
                  results={run.results}
                  selectedStrategy={selectedStrategy}
                  onSelectStrategy={handleToggleStrategy}
                />
              </section>
            )}
          </div>
        )}
      </main>
    </div>
  );
}
