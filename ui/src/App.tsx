import { useEffect, useState } from "react";
import { DrawdownChart } from "./components/DrawdownChart.tsx";
import { EquityChart } from "./components/EquityChart.tsx";
import { PriceChart } from "./components/PriceChart.tsx";
import { StrategyTable } from "./components/StrategyTable.tsx";
import type { BacktestRun } from "./types.ts";

export function App() {
  const [run, setRun] = useState<BacktestRun | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    fetch("/results.json")
      .then((res) => {
        if (!res.ok) {
          throw new Error(`${res.status} ${res.statusText}`);
        }
        return res.json() as Promise<BacktestRun>;
      })
      .then((data) => {
        if (!cancelled) {
          setError(null);
          setRun(data);
        }
      })
      .catch((e: unknown) => {
        if (!cancelled) {
          const message =
            e instanceof Error ? e.message : "Failed to load results";
          setError(message);
          setRun(null);
        }
      });

    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <div className="min-h-dvh bg-slate-950 text-slate-100">
      <header className="border-b border-slate-800 px-6 py-5">
        <h1 className="text-xl font-semibold tracking-tight text-slate-50">
          Rust Trader
        </h1>
        <p className="mt-1 text-sm text-slate-500">
          Backtest overview — data from{" "}
          <code className="text-slate-400">outputs/results.json</code>
        </p>
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

        {run === null && error === null && (
          <p className="text-sm text-slate-500">Loading backtest results…</p>
        )}

        {run !== null && (
          <>
            <section className="space-y-3">
              <h2 className="text-sm font-medium uppercase tracking-wide text-slate-500">
                Market Price
              </h2>
              <PriceChart run={run} />
            </section>

            <section className="space-y-3">
              <h2 className="text-sm font-medium uppercase tracking-wide text-slate-500">
                Strategy comparison
              </h2>
              <StrategyTable results={run.results} />
            </section>

            <section className="space-y-3">
              <h2 className="text-sm font-medium uppercase tracking-wide text-slate-500">
                Equity curves
              </h2>
              <EquityChart market={run.market} results={run.results} />
            </section>

            <section className="space-y-3">
              <h2 className="text-sm font-medium uppercase tracking-wide text-slate-500">
                Drawdown
              </h2>
              <DrawdownChart market={run.market} results={run.results} />
            </section>
          </>
        )}
      </main>
    </div>
  );
}
