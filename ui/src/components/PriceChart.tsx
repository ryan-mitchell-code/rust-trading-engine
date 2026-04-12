import {
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import type { MarketSeries } from "../types";
import {
  formatChartTooltipTimestamp,
  formatChartXAxisTickLabel,
  lineChartXMarginBottom,
  xAxisMinTickGap,
  xAxisTickStyle,
} from "./chartXAxis.ts";

type PriceChartProps = {
  market: MarketSeries;
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

/** `[timestamp, close]` → rows for Recharts. Does not mutate `market`. */
function marketToChartData(
  market: MarketSeries,
): { timestamp: string; price: number }[] {
  return market.map(([timestamp, price]) => ({ timestamp, price }));
}

export function PriceChart({ market }: PriceChartProps) {
  if (market.length === 0) {
    return (
      <p className="text-sm text-slate-500">No market data to plot.</p>
    );
  }

  const chartData = marketToChartData(market);
  const pointCount = chartData.length;

  return (
    <div className="h-80 w-full rounded-lg border border-slate-800 bg-slate-900/40 p-2">
      <ResponsiveContainer width="100%" height="100%">
        <LineChart
          data={chartData}
          margin={{
            top: 8,
            right: 12,
            bottom: lineChartXMarginBottom(pointCount),
            left: 8,
          }}
        >
          <XAxis
            dataKey="timestamp"
            stroke="#64748b"
            tick={xAxisTickStyle(pointCount)}
            minTickGap={xAxisMinTickGap(pointCount)}
            tickFormatter={(value) =>
              typeof value === "string"
                ? formatChartXAxisTickLabel(value)
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
            labelFormatter={(label) => formatChartTooltipTimestamp(label)}
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
