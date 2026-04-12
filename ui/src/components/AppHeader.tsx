import { shellMaxClass } from "../constants/layout.ts";
import type { Dataset } from "../services/api.ts";

type AppHeaderProps = {
  dataset: Dataset;
  loading: boolean;
  onDatasetChange: (dataset: Dataset) => void;
  onRunBacktest: () => void;
};

export function AppHeader({
  dataset,
  loading,
  onDatasetChange,
  onRunBacktest,
}: AppHeaderProps) {
  return (
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
            onChange={(e) => onDatasetChange(e.target.value as Dataset)}
            disabled={loading}
            className="select-inset-chevron rounded-lg border border-slate-700 bg-slate-950/80 py-2 pl-3 pr-10 text-sm text-slate-100 shadow-sm outline-none ring-sky-500/30 transition focus:border-sky-600 focus:ring-2 disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value="BTCUSDT">BTCUSDT</option>
            <option value="ETHUSDT">ETHUSDT</option>
          </select>
          <button
            type="button"
            onClick={onRunBacktest}
            disabled={loading}
            className="rounded-lg bg-sky-600 px-4 py-2 text-sm font-medium text-white shadow-sm transition hover:bg-sky-500 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {loading ? "Running…" : "Run Backtest"}
          </button>
        </div>
      </div>
    </header>
  );
}
