import { cn } from "../../lib/cn.ts";
import type { ChartRangePreset } from "./CandlestickChart.tsx";

const OPTIONS: { id: ChartRangePreset; label: string }[] = [
  { id: "all", label: "All" },
  { id: "1m", label: "1M" },
  { id: "3m", label: "3M" },
  { id: "6m", label: "6M" },
  { id: "1y", label: "1Y" },
];

type ChartRangeControlsProps = {
  value: ChartRangePreset;
  onChange: (preset: ChartRangePreset) => void;
  disabled?: boolean;
};

export function ChartRangeControls({
  value,
  onChange,
  disabled = false,
}: ChartRangeControlsProps) {
  return (
    <div
      className="flex flex-wrap items-center gap-1.5"
      role="group"
      aria-label="Chart time range"
    >
      <span className="text-xs text-slate-500">Range</span>
      {OPTIONS.map((opt) => (
        <button
          key={opt.id}
          type="button"
          disabled={disabled}
          onClick={() => onChange(opt.id)}
          className={cn(
            "rounded-md border px-2 py-1 text-xs font-medium transition disabled:cursor-not-allowed disabled:opacity-50",
            value === opt.id
              ? "border-sky-600 bg-sky-950/50 text-sky-200"
              : "border-slate-700 bg-slate-900/60 text-slate-400 hover:border-slate-600 hover:text-slate-300",
          )}
        >
          {opt.label}
        </button>
      ))}
    </div>
  );
}
