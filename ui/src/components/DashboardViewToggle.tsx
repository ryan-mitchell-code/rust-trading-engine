export type DashboardView = "charts" | "table";

type DashboardViewToggleProps = {
  view: DashboardView;
  onViewChange: (view: DashboardView) => void;
  chartsTabId: string;
  tableTabId: string;
  chartsPanelId: string;
  tablePanelId: string;
  /** Accessible name for the tablist (e.g. ties to surrounding section). */
  "aria-label"?: string;
};

export function DashboardViewToggle({
  view,
  onViewChange,
  chartsTabId,
  tableTabId,
  chartsPanelId,
  tablePanelId,
  "aria-label": ariaLabel = "Dashboard view",
}: DashboardViewToggleProps) {
  return (
    <div
      className="inline-flex shrink-0 rounded-lg border border-slate-800 bg-slate-900/60 p-1 shadow-sm"
      role="tablist"
      aria-label={ariaLabel}
    >
      <button
        id={chartsTabId}
        type="button"
        role="tab"
        aria-selected={view === "charts"}
        aria-controls={chartsPanelId}
        onClick={() => onViewChange("charts")}
        className={`rounded-md px-4 py-2 text-sm font-medium transition ${
          view === "charts"
            ? "bg-sky-600 text-white shadow-sm ring-1 ring-sky-400/40"
            : "text-slate-400 hover:text-slate-200"
        }`}
      >
        Charts
      </button>
      <button
        id={tableTabId}
        type="button"
        role="tab"
        aria-selected={view === "table"}
        aria-controls={tablePanelId}
        onClick={() => onViewChange("table")}
        className={`rounded-md px-4 py-2 text-sm font-medium transition ${
          view === "table"
            ? "bg-sky-600 text-white shadow-sm ring-1 ring-sky-400/40"
            : "text-slate-400 hover:text-slate-200"
        }`}
      >
        Table
      </button>
    </div>
  );
}
