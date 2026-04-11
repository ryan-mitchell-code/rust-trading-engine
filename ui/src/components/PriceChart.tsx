import {
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import type { BacktestRun } from "../types";

type PriceChartProps = {
  run: BacktestRun;
};

/** Single-series stroke: visible on dark bg, neutral slate–sky. */
const MARKET_LINE_STROKE = "#94a3b8";

function formatPriceAxis(n: number): string {
  return n.toLocaleString(undefined, {
    maximumFractionDigits: 2,
  });
}

function formatPriceTooltip(n: number): string {
  return n.toLocaleString(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

/** Shorter axis labels when many points; full string if unparsable as a date. */
function formatXAxisTickLabel(raw: string): string {
  const t = new Date(raw).getTime();
  if (Number.isNaN(t)) {
    return raw.length > 12 ? `${raw.slice(0, 10)}…` : raw;
  }
  return new Date(t).toLocaleDateString(undefined, {
    month: "short",
    day: "numeric",
  });
}

function formatTooltipTimestamp(raw: unknown): string {
  const s = String(raw);
  const t = new Date(s).getTime();
  if (Number.isNaN(t)) {
    return s;
  }
  return new Date(t).toLocaleString(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  });
}

/** `[timestamp, close]` → rows for Recharts. Does not mutate `run`. */
function marketToChartData(
  run: BacktestRun,
): { timestamp: string; price: number }[] {
  return run.market.map(([timestamp, price]) => ({ timestamp, price }));
}

export function PriceChart({ run }: PriceChartProps) {
  if (run.market.length === 0) {
    return (
      <p className="text-sm text-slate-500">No market data to plot.</p>
    );
  }

  const chartData = marketToChartData(run);
  const pointCount = chartData.length;
  const hideXTickLabels = pointCount > 360;
  const xAxisDense = pointCount > 80 && !hideXTickLabels;

  return (
    <div className="h-80 w-full rounded-lg border border-slate-800 bg-slate-900/40 p-2">
      <ResponsiveContainer width="100%" height="100%">
        <LineChart
          data={chartData}
          margin={{
            top: 8,
            right: 12,
            bottom: (hideXTickLabels ? 12 : xAxisDense ? 20 : 16) + 8,
            left: 8,
          }}
        >
          <XAxis
            dataKey="timestamp"
            stroke="#64748b"
            tick={
              hideXTickLabels
                ? false
                : { fill: "#94a3b8", fontSize: 10 }
            }
            minTickGap={hideXTickLabels ? undefined : xAxisDense ? 52 : 28}
            tickFormatter={(value) =>
              typeof value === "string"
                ? formatXAxisTickLabel(value)
                : String(value)
            }
          />
          <YAxis
            type="number"
            stroke="#64748b"
            tick={{ fill: "#94a3b8", fontSize: 11 }}
            tickFormatter={(v) =>
              typeof v === "number" ? formatPriceAxis(v) : String(v)
            }
          />
          <Tooltip
            contentStyle={{
              backgroundColor: "#0f172a",
              border: "1px solid #334155",
              borderRadius: "8px",
            }}
            labelStyle={{ color: "#e2e8f0" }}
            itemStyle={{ color: "#e2e8f0" }}
            formatter={(value) =>
              typeof value === "number"
                ? formatPriceTooltip(value)
                : String(value)
            }
            labelFormatter={(label) => formatTooltipTimestamp(label)}
          />
          <Line
            type="monotone"
            dataKey="price"
            name="Price"
            stroke={MARKET_LINE_STROKE}
            strokeWidth={2}
            strokeLinecap="round"
            strokeLinejoin="round"
            dot={false}
            connectNulls
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
