import {
  Legend,
  Line,
  LineChart,
  ReferenceLine,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import type { BacktestResult, MarketSeries } from "../types";
import {
  BENCHMARK_STRATEGY_NAME,
  BENCHMARK_STROKE,
  BENCHMARK_STROKE_DASHARRAY,
  BENCHMARK_STROKE_WIDTH,
  LINE_STROKE_WIDTH,
  LINE_STROKE_WIDTH_HIGHLIGHT,
  SERIES_COLORS,
} from "../constants/chartTheme.ts";
import {
  formatChartTooltipTimestamp,
  formatChartXAxisTickLabel,
  lineChartXMarginBottom,
  xAxisMinTickGap,
  xAxisTickStyle,
} from "./chartXAxis.ts";

/** Non-benchmark series: slightly transparent so benchmark reads clearly. */
const ACTIVE_STRATEGY_STROKE_OPACITY = 0.7;

/** Non-selected series when a strategy is focused. */
const DIM_OPACITY = 0.3;

type DrawdownChartProps = {
  market: MarketSeries;
  results: BacktestResult[];
  selectedStrategy?: string | null;
};

/** Backend drawdown ratios → percentage strings (2 decimals). */
function formatDrawdownPercent(n: number): string {
  return `${(n * 100).toFixed(2)}%`;
}

/** Most negative drawdown ratio across all series (worst point). */
function minDrawdownAcrossStrategies(
  chartData: Record<string, string | number>[],
  strategyNames: string[],
): number {
  let minDd = 0;
  for (const row of chartData) {
    for (const name of strategyNames) {
      const v = row[name];
      if (typeof v === "number" && Number.isFinite(v)) {
        minDd = Math.min(minDd, v);
      }
    }
  }
  return minDd;
}

/**
 * Y domain: top fixed at 0 (peak); bottom from worst drawdown with ~10% padding below.
 * Degenerate flat-at-peak case uses a thin band so the axis is usable (similar spirit to EquityChart padding).
 */
function drawdownYDomain(minDd: number): [number, number] {
  if (minDd >= 0) {
    return [-0.01, 0];
  }
  return [minDd * 1.1, 0];
}

/**
 * One row per bar; `timestamp` from `market`, plus one field per strategy `name` (drawdown ratio).
 * Uses backend `drawdown_curve` only — no drawdown math here.
 */
function mergeDrawdownCurves(
  market: MarketSeries,
  results: BacktestResult[],
): Record<string, string | number>[] {
  const lengths = results.map((r) => r.drawdown_curve.length);
  const n = Math.min(market.length, ...lengths);
  const rows: Record<string, string | number>[] = [];
  for (let i = 0; i < n; i++) {
    const timestamp = market[i][0];
    const row: Record<string, string | number> = { timestamp };
    for (const r of results) {
      row[r.name] = r.drawdown_curve[i];
    }
    rows.push(row);
  }
  return rows;
}

export function DrawdownChart({
  market,
  results,
  selectedStrategy = null,
}: DrawdownChartProps) {
  const withCurves = results.filter((r) => r.drawdown_curve.length > 0);
  if (withCurves.length === 0 || market.length === 0) {
    return (
      <p className="text-sm text-slate-500">No drawdown series to plot.</p>
    );
  }

  const chartData = mergeDrawdownCurves(market, withCurves);
  const strategyNames = withCurves.map((r) => r.name);
  const yDomain = drawdownYDomain(
    minDrawdownAcrossStrategies(chartData, strategyNames),
  );
  const topCapitalName = withCurves.reduce((best, r) =>
    r.summary.final_capital > best.summary.final_capital ? r : best,
  ).name;
  const pointCount = chartData.length;

  const hasSelection =
    selectedStrategy != null && selectedStrategy !== "";

  let activePaletteIndex = 0;
  const drawdownLines = strategyNames.map((name) => {
    const isBenchmark = name === BENCHMARK_STRATEGY_NAME;
    const stroke = isBenchmark
      ? BENCHMARK_STROKE
      : SERIES_COLORS[activePaletteIndex++ % SERIES_COLORS.length];
    const isTopCapital = !isBenchmark && name === topCapitalName;
    const isHighlighted = hasSelection && name === selectedStrategy;

    let strokeOpacity: number;
    let strokeWidth: number;
    if (hasSelection) {
      strokeOpacity = isHighlighted ? 1 : DIM_OPACITY;
      strokeWidth = isHighlighted
        ? LINE_STROKE_WIDTH_HIGHLIGHT
        : isBenchmark
          ? BENCHMARK_STROKE_WIDTH
          : LINE_STROKE_WIDTH;
    } else {
      strokeOpacity = isBenchmark ? 1 : ACTIVE_STRATEGY_STROKE_OPACITY;
      strokeWidth = isBenchmark
        ? BENCHMARK_STROKE_WIDTH
        : isTopCapital
          ? LINE_STROKE_WIDTH_HIGHLIGHT
          : LINE_STROKE_WIDTH;
    }

    return (
      <Line
        key={name}
        type="monotone"
        dataKey={name}
        name={isBenchmark ? `${name} (benchmark)` : name}
        stroke={stroke}
        strokeOpacity={strokeOpacity}
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
            domain={yDomain}
            stroke="#64748b"
            tick={{ fill: "#94a3b8", fontSize: 11 }}
            tickFormatter={(v) =>
              typeof v === "number" ? formatDrawdownPercent(v) : String(v)
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
                ? formatDrawdownPercent(value)
                : String(value)
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
          <ReferenceLine
            y={0}
            stroke="#94a3b8"
            strokeDasharray="4 4"
            strokeOpacity={0.85}
          />
          {drawdownLines}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
