import { useCallback, useId, useState } from "react";
import { AppHeader } from "./components/AppHeader.tsx";
import { ChartSection } from "./components/ChartSection.tsx";
import {
  DashboardViewToggle,
  type DashboardView,
} from "./components/DashboardViewToggle.tsx";
import { DrawdownChart } from "./components/DrawdownChart.tsx";
import { EquityChart } from "./components/EquityChart.tsx";
import { HelpHint } from "./components/HelpHint.tsx";
import { CandlestickChart } from "./components/CandlestickChart.tsx";
import { StrategyMetricCards } from "./components/StrategyMetricCards.tsx";
import { StrategyTable } from "./components/StrategyTable.tsx";
import { cardClass, shellMaxClass } from "./constants/layout.ts";
import { bestStrategyNameByReturn } from "./lib/backtestMetrics.ts";
import {
  BACKTEST_INTERVAL,
  fetchBacktestRun,
  type Dataset,
} from "./services/api.ts";
import type { BacktestRun } from "./types.ts";

export function App() {
  const [run, setRun] = useState<BacktestRun | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [dataset, setDataset] = useState<Dataset>("BTCUSDT");
  const [maShort, setMaShort] = useState(10);
  const [maLong, setMaLong] = useState(50);
  const [view, setView] = useState<DashboardView>("charts");
  const [selectedStrategy, setSelectedStrategy] = useState<string | null>(null);

  const viewIds = useId();
  const chartsTabId = `${viewIds}-charts-tab`;
  const tableTabId = `${viewIds}-table-tab`;
  const chartsPanelId = `${viewIds}-charts-panel`;
  const tablePanelId = `${viewIds}-table-panel`;

  const handleToggleStrategy = useCallback((strategyName: string) => {
    setSelectedStrategy((prev) =>
      prev === strategyName ? null : strategyName,
    );
  }, []);

  const handleRunBacktest = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await fetchBacktestRun(dataset, {
        short: maShort,
        long: maLong,
      });
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
  }, [dataset, maShort, maLong]);

  const bestStrategyName =
    run !== null ? bestStrategyNameByReturn(run.results) : null;

  return (
    <div className="min-h-dvh bg-slate-950 text-slate-100">
      <AppHeader
        dataset={dataset}
        maShort={maShort}
        maLong={maLong}
        loading={loading}
        onDatasetChange={setDataset}
        onMaShortChange={setMaShort}
        onMaLongChange={setMaLong}
        onRunBacktest={handleRunBacktest}
      />

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
          <div className={`${cardClass}`}>
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
            <ChartSection title="Market Price" dataset={dataset}>
              <CandlestickChart
                market={run.market}
                interval={BACKTEST_INTERVAL}
                trades={
                  selectedStrategy === null
                    ? undefined
                    : run.results.find((r) => r.name === selectedStrategy)
                        ?.trades
                }
              />
            </ChartSection>

            <StrategyMetricCards
              results={run.results}
              selectedStrategy={selectedStrategy}
              bestStrategyName={bestStrategyName}
              onToggleStrategy={handleToggleStrategy}
            />

            <section className={`space-y-5 ${cardClass}`} aria-label="Performance">
              <div className="flex flex-col gap-3 border-b border-slate-800/90 pb-4 sm:flex-row sm:items-end sm:justify-between sm:gap-4">
                <div className="min-w-0">
                  <h2 className="text-sm font-semibold tracking-tight text-slate-200">
                    Performance
                  </h2>
                  <p className="mt-1 text-xs leading-relaxed text-slate-500">
                    Equity and drawdown charts, or the full metrics table — same
                    strategy focus as the cards above.
                  </p>
                </div>
                <DashboardViewToggle
                  view={view}
                  onViewChange={setView}
                  chartsTabId={chartsTabId}
                  tableTabId={tableTabId}
                  chartsPanelId={chartsPanelId}
                  tablePanelId={tablePanelId}
                  aria-label="Charts or table layout"
                />
              </div>

              <div
                id={chartsPanelId}
                role="tabpanel"
                aria-labelledby={chartsTabId}
                hidden={view !== "charts"}
              >
                {view === "charts" && (
                  <div className="grid grid-cols-1 gap-8 md:grid-cols-2">
                    <ChartSection title="Equity curves" dataset={dataset}>
                      <EquityChart
                        market={run.market}
                        results={run.results}
                        selectedStrategy={selectedStrategy}
                      />
                    </ChartSection>

                    <ChartSection title="Drawdown" dataset={dataset}>
                      <DrawdownChart
                        market={run.market}
                        results={run.results}
                        selectedStrategy={selectedStrategy}
                      />
                    </ChartSection>
                  </div>
                )}
              </div>

              <div
                id={tablePanelId}
                role="tabpanel"
                aria-labelledby={tableTabId}
                hidden={view !== "table"}
              >
                {view === "table" && (
                  <div className="space-y-4">
                    <div className="flex items-start justify-between gap-3">
                      <h3 className="text-sm font-semibold tracking-tight text-slate-200">
                        Strategy comparison
                      </h3>
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
                  </div>
                )}
              </div>
            </section>
          </div>
        )}
      </main>
    </div>
  );
}
