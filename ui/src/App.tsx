import { useCallback, useState } from "react";
import { DrawdownChart } from "./components/DrawdownChart.tsx";
import { EquityChart } from "./components/EquityChart.tsx";
import { PriceChart } from "./components/PriceChart.tsx";
import { StrategyTable } from "./components/StrategyTable.tsx";
import type { BacktestResult, BacktestRun } from "./types.ts";

function findSelectedResult(
  results: BacktestResult[],
  selectedStrategy: string | null,
): BacktestResult | undefined {
  if (selectedStrategy === null) return undefined;
  return results.find((r) => r.name === selectedStrategy);
}

const RUN_BODY = {
  dataset: "BTCUSDT",
  interval: "1d",
} as const;

async function fetchBacktestRun(): Promise<BacktestRun> {
  const res = await fetch("/run", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(RUN_BODY),
  });
  if (!res.ok) {
    throw new Error(`${res.status} ${res.statusText}`);
  }
  return res.json() as Promise<BacktestRun>;
}

export function App() {
  const [run, setRun] = useState<BacktestRun | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
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
      const data = await fetchBacktestRun();
      setSelectedStrategy(null);
      setRun(data);
    } catch (e: unknown) {
      const message =
        e instanceof Error ? e.message : "Failed to run backtest";
      setError(message);
      setSelectedStrategy(null);
      setRun(null);
    } finally {
      setLoading(false);
    }
  }, []);

  const selected =
    run !== null
      ? findSelectedResult(run.results, selectedStrategy)
      : undefined;
  const tradesForChart = selected ? selected.trades : undefined;

  return (
    <div className="min-h-dvh bg-slate-950 text-slate-100">
      <header className="border-b border-slate-800 px-6 py-5">
        <div className="flex flex-wrap items-start justify-between gap-4">
          <div>
            <h1 className="text-xl font-semibold tracking-tight text-slate-50">
              Rust Trader
            </h1>
            <p className="mt-1 text-sm text-slate-500">
              Backtest overview — data from the API (
              <code className="text-slate-400">POST /run</code>)
            </p>
          </div>
          <button
            type="button"
            onClick={handleRunBacktest}
            disabled={loading}
            className="rounded-lg bg-sky-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-sky-500 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {loading ? "Running…" : "Run Backtest"}
          </button>
        </div>
      </header>

      <main className="mx-auto max-w-6xl space-y-10 px-6 py-8">
        {error !== null && (
          <div
            className="rounded-lg border border-amber-900/60 bg-amber-950/40 px-4 py-3 text-sm text-amber-200"
            role="alert"
          >
            Could not load backtests: {error}
          </div>
        )}

        {loading && (
          <p className="text-sm text-slate-500">Running backtest…</p>
        )}

        {!loading && run === null && error === null && (
          <p className="text-sm text-slate-500">
            Click <span className="text-slate-400">Run Backtest</span> to load
            results.
          </p>
        )}

        {run !== null && (
          <>
            <section className="space-y-3">
              <h2 className="text-sm font-medium uppercase tracking-wide text-slate-500">
                Market Price
              </h2>
              <PriceChart market={run.market} trades={tradesForChart} />
            </section>

            <section className="space-y-3">
              <div>
                <h2 className="text-sm font-medium uppercase tracking-wide text-slate-500">
                  Strategy comparison
                </h2>
                <p className="mt-1 text-xs text-slate-500">
                  Click a row to focus one strategy; click again to clear
                  selection.
                </p>
              </div>
              <StrategyTable
                results={run.results}
                selectedStrategy={selectedStrategy}
                onSelectStrategy={handleToggleStrategy}
              />
            </section>

            <section className="space-y-3">
              <h2 className="text-sm font-medium uppercase tracking-wide text-slate-500">
                Equity curves
              </h2>
              <EquityChart
                market={run.market}
                results={run.results}
                selectedStrategy={selectedStrategy}
              />
            </section>

            <section className="space-y-3">
              <h2 className="text-sm font-medium uppercase tracking-wide text-slate-500">
                Drawdown
              </h2>
              <DrawdownChart
                market={run.market}
                results={run.results}
                selectedStrategy={selectedStrategy}
              />
            </section>
          </>
        )}
      </main>
    </div>
  );
}
