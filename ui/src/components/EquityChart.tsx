import {
  Legend,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import type { BacktestResult, MarketSeries } from "../types";
import {
  formatChartTooltipTimestamp,
  formatChartXAxisTickLabel,
  lineChartXMarginBottom,
  xAxisMinTickGap,
  xAxisTickStyle,
} from "./chartXAxis.ts";

type EquityChartProps = {
  market: MarketSeries;
  results: BacktestResult[];
};

/** Fixed order: distinct on dark backgrounds, consistent across Line + Legend. */
export const SERIES_COLORS = [
  "#38bdf8",
  "#818cf8",
  "#34d399",
  "#fbbf24",
  "#f472b6",
] as const;

export const LINE_STROKE_WIDTH = 2;
export const LINE_STROKE_WIDTH_HIGHLIGHT = 3;

/** Matches `BUY_AND_HOLD_NAME` in backend — passive benchmark, not an active strategy. */
export const BENCHMARK_STRATEGY_NAME = "buy_and_hold";
export const BENCHMARK_STROKE = "#94a3b8";
/** Thin stroke so dash pattern reads as dotted vs solid active series. */
export const BENCHMARK_STROKE_WIDTH = 1.15;
/** Short gaps + small dashes → visibly dotted (with round line caps). */
export const BENCHMARK_STROKE_DASHARRAY = "1.5 5";

/** Capital display: matches Y-axis ticks and tooltip values. */
function formatCapital(n: number): string {
  return n.toLocaleString(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

/** Tight Y-axis around the plotted capitals: small padding so lines aren’t clipped at the edges. */
function paddedCapitalDomain(
  domain: readonly [number, number],
): [number, number] {
  const dataMin = domain[0];
  const dataMax = domain[1];
  const span = dataMax - dataMin || Math.max(Math.abs(dataMin) * 0.001, 1);
  const pad = span * 0.05;
  return [dataMin - pad, dataMax + pad];
}

/**
 * One row per bar; `timestamp` from `market`, plus one numeric field per strategy `name` (capital).
 * Curves are aligned by index with `market` — does not mutate inputs.
 */
function mergeEquityCurves(
  market: MarketSeries,
  results: BacktestResult[],
): Record<string, string | number>[] {
  const lengths = results.map((r) => r.equity_curve.length);
  const n = Math.min(market.length, ...lengths);
  const rows: Record<string, string | number>[] = [];
  for (let i = 0; i < n; i++) {
    const timestamp = market[i][0];
    const row: Record<string, string | number> = { timestamp };
    for (const r of results) {
      row[r.name] = r.equity_curve[i];
    }
    rows.push(row);
  }
  return rows;
}

export function EquityChart({ market, results }: EquityChartProps) {
  const withCurves = results.filter((r) => r.equity_curve.length > 0);
  if (withCurves.length === 0 || market.length === 0) {
    return (
      <p className="text-sm text-slate-500">No equity curves to plot.</p>
    );
  }

  const chartData = mergeEquityCurves(market, withCurves);
  const strategyNames = withCurves.map((r) => r.name);
  const topCapitalName = withCurves.reduce((best, r) =>
    r.summary.final_capital > best.summary.final_capital ? r : best,
  ).name;
  const pointCount = chartData.length;

  let activePaletteIndex = 0;
  const equityLines = strategyNames.map((name) => {
    const isBenchmark = name === BENCHMARK_STRATEGY_NAME;
    const stroke = isBenchmark
      ? BENCHMARK_STROKE
      : SERIES_COLORS[activePaletteIndex++ % SERIES_COLORS.length];
    const isTopCapital = !isBenchmark && name === topCapitalName;
    const strokeWidth = isBenchmark
      ? BENCHMARK_STROKE_WIDTH
      : isTopCapital
        ? LINE_STROKE_WIDTH_HIGHLIGHT
        : LINE_STROKE_WIDTH;
    return (
      <Line
        key={name}
        type="monotone"
        dataKey={name}
        name={isBenchmark ? `${name} (benchmark)` : name}
        stroke={stroke}
        strokeWidth={strokeWidth}
        strokeDasharray={
          isBenchmark ? BENCHMARK_STROKE_DASHARRAY : undefined
        }
        strokeLinecap="round"
        strokeLinejoin="round"
        dot={false}
        connectNulls
      />
    );
  });

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
            domain={paddedCapitalDomain}
            stroke="#64748b"
            tick={{ fill: "#94a3b8", fontSize: 11 }}
            tickFormatter={(v) =>
              typeof v === "number" ? formatCapital(v) : String(v)
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
              typeof value === "number" ? formatCapital(value) : String(value)
            }
            labelFormatter={(label) => formatChartTooltipTimestamp(label)}
          />
          <Legend
            verticalAlign="bottom"
            iconType="plainline"
            iconSize={14}
            wrapperStyle={{
              paddingTop: 16,
              fontSize: 13,
              lineHeight: "1.45",
              color: "#cbd5e1",
              letterSpacing: "0.01em",
            }}
          />
          {equityLines}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
