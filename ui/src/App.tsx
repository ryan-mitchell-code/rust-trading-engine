import { useCallback, useId, useRef, useState } from "react";
import type { IChartApi } from "lightweight-charts";
import { AppHeader } from "./components/AppHeader.tsx";
import {
  applyChartRangePreset,
  type ChartRangePreset,
  CandlestickChart,
} from "./components/charts/CandlestickChart.tsx";
import { ChartRangeControls } from "./components/charts/ChartRangeControls.tsx";
import { ChartSection } from "./components/charts/ChartSection.tsx";
import { DrawdownChart } from "./components/charts/DrawdownChart.tsx";
import { EquityChart } from "./components/charts/EquityChart.tsx";
import {
  DashboardViewToggle,
  type DashboardView,
} from "./components/DashboardViewToggle.tsx";
import { HelpHint } from "./components/HelpHint.tsx";
import { StrategyMetricCards } from "./components/results/StrategyMetricCards.tsx";
import { StrategyTable } from "./components/results/StrategyTable.tsx";
import { StrategyPanel } from "./components/strategy-panel/StrategyPanel.tsx";
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
  const [maParams, setMaParams] = useState({ short: 10, long: 50 });
  const [rsiParams, setRsiParams] = useState({
    period: 14,
    overbought: 70,
    oversold: 30,
  });
  const [showSettings, setShowSettings] = useState(false);
  const [view, setView] = useState<DashboardView>("charts");
  const [selectedStrategy, setSelectedStrategy] = useState<string | null>(null);
  const [candleRange, setCandleRange] = useState<ChartRangePreset>("all");
  const candleRangeRef = useRef<ChartRangePreset>(candleRange);
  candleRangeRef.current = candleRange;
  const candlestickChartRef = useRef<IChartApi | null>(null);

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
      const data = await fetchBacktestRun(dataset, maParams, rsiParams);
      setCandleRange("all");
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
  }, [dataset, maParams, rsiParams]);

  const bestStrategyName =
    run !== null ? bestStrategyNameByReturn(run.results) : null;

  const handleCandlestickChartReady = useCallback((chart: IChartApi | null) => {
    candlestickChartRef.current = chart;
    if (chart !== null && run !== null) {
      applyChartRangePreset(
        chart,
        run.market,
        BACKTEST_INTERVAL,
        candleRangeRef.current,
      );
    }
  }, [run]);

  const handleCandleRangeChange = useCallback(
    (preset: ChartRangePreset) => {
      setCandleRange(preset);
      const chart = candlestickChartRef.current;
      if (chart === null || run === null) return;
      applyChartRangePreset(chart, run.market, BACKTEST_INTERVAL, preset);
    },
    [run],
  );

  return (
    <div className="min-h-dvh bg-slate-950 text-slate-100">
      <AppHeader
        dataset={dataset}
        loading={loading}
        showSettings={showSettings}
        onDatasetChange={setDataset}
        onToggleSettings={() => setShowSettings((v) => !v)}
        onRunBacktest={handleRunBacktest}
      />

      <main
        className={`space-y-8 px-4 py-6 sm:py-8 lg:px-6 lg:py-10 xl:px-8 ${shellMaxClass}`}
      >
        {!showSettings && (
          <p
            className="text-xs text-slate-500 tabular-nums"
            aria-live="polite"
          >
            <span className="text-slate-600">MA</span>{" "}
            {maParams.short}/{maParams.long}
            <span className="mx-2 text-slate-700">·</span>
            <span className="text-slate-600">RSI</span> P{rsiParams.period}{" "}
            OB{rsiParams.overbought} OS{rsiParams.oversold}
          </p>
        )}

        {showSettings && (
          <div
            className="rounded-xl border border-slate-800/80 bg-slate-900/95 p-4 shadow-md shadow-black/25 ring-1 ring-white/[0.06] sm:p-5"
            role="region"
            aria-label="Strategy parameter editor"
          >
            <StrategyPanel
              loading={loading}
              maParams={maParams}
              rsiParams={rsiParams}
              onMaShortChange={(short) =>
                setMaParams((p) => ({ ...p, short }))
              }
              onMaLongChange={(long) => setMaParams((p) => ({ ...p, long }))}
              onRsiPeriodChange={(period) =>
                setRsiParams((p) => ({ ...p, period }))
              }
              onRsiOverboughtChange={(overbought) =>
                setRsiParams((p) => ({ ...p, overbought }))
              }
              onRsiOversoldChange={(oversold) =>
                setRsiParams((p) => ({ ...p, oversold }))
              }
            />
          </div>
        )}

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
              <div className="mb-3 flex flex-wrap items-center justify-between gap-2 border-b border-slate-800/60 pb-3">
                <ChartRangeControls
                  value={candleRange}
                  onChange={handleCandleRangeChange}
                />
              </div>
              <CandlestickChart
                market={run.market}
                interval={BACKTEST_INTERVAL}
                trades={
                  selectedStrategy === null
                    ? undefined
                    : run.results.find((r) => r.name === selectedStrategy)
                        ?.trades
                }
                onChartReady={handleCandlestickChartReady}
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
