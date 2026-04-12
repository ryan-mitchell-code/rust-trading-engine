import {
  ComposedChart,
  Line,
  ResponsiveContainer,
  Scatter,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import type { MarketSeries } from "../types";
import {
  formatChartTooltipTimestamp,
  formatChartXAxisTickLabel,
  lineChartXMarginBottom,
  parseChartTimestamp,
  xAxisMinTickGap,
  xAxisTickStyle,
} from "./chartXAxis.ts";

type PriceChartProps = {
  market: MarketSeries;
  /** Trade rows from one strategy: [trade_id, timestamp, side, price, pnl, capital][] */
  trades?: string[][];
};

/** Single-series stroke: visible on dark bg, neutral slate–sky. */
const MARKET_LINE_STROKE = "#94a3b8";

const BUY_FILL = "#22c55e";
const SELL_FILL = "#ef4444";

/** Y offset from trade price so markers sit slightly below (buy) or above (sell) the line. */
const MARKER_OFFSET_FACTOR = 0.0015;

type ParsedTrade = {
  tradeTimestamp: string;
  side: "BUY" | "SELL";
  price: number;
};

type ChartRow = {
  timestamp: string;
  time: number;
  price: number;
};

type TradeMarkerPoint = {
  time: number;
  markerY: number;
  side: "BUY" | "SELL";
  price: number;
  timestamp: string;
};

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

function parseStrategyTrades(trades: string[][]): ParsedTrade[] {
  const out: ParsedTrade[] = [];
  for (const row of trades) {
    if (row.length < 4) continue;
    const [, ts, sideRaw, priceRaw] = row;
    const side = sideRaw.trim().toUpperCase();
    if (side !== "BUY" && side !== "SELL") continue;
    const price = Number(priceRaw);
    if (!Number.isFinite(price)) continue;
    out.push({
      tradeTimestamp: ts,
      side: side as ParsedTrade["side"],
      price,
    });
  }
  return out;
}

function buildTradeMarkers(trades: string[][]): TradeMarkerPoint[] {
  const parsed = parseStrategyTrades(trades);
  if (parsed.length === 0) return [];

  const markers: TradeMarkerPoint[] = [];
  for (const t of parsed) {
    const d = parseChartTimestamp(t.tradeTimestamp);
    if (!d) continue;

    const time = d.getTime();
    const factor =
      t.side === "BUY" ? 1 - MARKER_OFFSET_FACTOR : 1 + MARKER_OFFSET_FACTOR;
    markers.push({
      time,
      markerY: t.price * factor,
      side: t.side,
      price: t.price,
      timestamp: t.tradeTimestamp,
    });
  }
  return markers;
}

/** Parse each bar into numeric time (ms) for a linear time axis. */
function marketToChartData(market: MarketSeries): ChartRow[] {
  const rows: ChartRow[] = [];
  for (const [timestamp, price] of market) {
    const d = parseChartTimestamp(timestamp);
    if (!d) continue;
    rows.push({
      timestamp,
      time: d.getTime(),
      price,
    });
  }
  return rows;
}

/** X-axis ticks are numeric ms; reuse string-based date formatting. */
function formatXAxisTickFromTime(value: unknown): string {
  if (typeof value === "number" && Number.isFinite(value)) {
    return formatChartXAxisTickLabel(String(Math.round(value)));
  }
  return String(value);
}

function isTradeMarkerPayload(p: unknown): p is TradeMarkerPoint {
  if (!p || typeof p !== "object") return false;
  const o = p as { side?: string };
  return o.side === "BUY" || o.side === "SELL";
}

function PriceChartTooltip({
  active,
  payload,
}: {
  active?: boolean;
  payload?: Array<{ payload?: unknown; value?: number }>;
}) {
  if (!active || !payload?.length) return null;

  const row = payload[0]?.payload;
  if (isTradeMarkerPayload(row)) {
    return (
      <div
        className="rounded-lg border border-slate-700 px-3 py-2 text-xs shadow-lg"
        style={{
          backgroundColor: "#0f172a",
          borderColor: "#334155",
        }}
      >
        <div className="font-medium text-slate-100">{row.side}</div>
        <div className="mt-1 text-slate-300">
          {formatPriceTooltip(row.price)}
        </div>
        <div className="mt-0.5 text-slate-400">
          {formatChartTooltipTimestamp(row.timestamp)}
        </div>
      </div>
    );
  }

  const line = row as { timestamp?: string; price?: number } | undefined;
  const price =
    typeof line?.price === "number"
      ? line.price
      : typeof payload[0]?.value === "number"
        ? payload[0].value
        : null;
  const ts = line?.timestamp;
  if (price === null) return null;

  return (
    <div
      className="rounded-lg border border-slate-700 px-3 py-2 text-xs shadow-lg"
      style={{
        backgroundColor: "#0f172a",
        borderColor: "#334155",
      }}
    >
      <div className="text-slate-300">{formatPriceTooltip(price)}</div>
      {ts !== undefined && (
        <div className="mt-0.5 text-slate-400">
          {formatChartTooltipTimestamp(ts)}
        </div>
      )}
    </div>
  );
}

export function PriceChart({ market, trades }: PriceChartProps) {
  if (market.length === 0) {
    return (
      <p className="text-sm text-slate-500">No market data to plot.</p>
    );
  }

  const chartData = marketToChartData(market);
  if (chartData.length === 0) {
    return (
      <p className="text-sm text-slate-500">No market data to plot.</p>
    );
  }

  const pointCount = chartData.length;

  const tradeMarkers =
    trades !== undefined && trades.length > 0 ? buildTradeMarkers(trades) : [];

  return (
    <div className="w-full rounded-lg border border-slate-800 bg-slate-900/40 p-2">
      <div className="h-80 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <ComposedChart
            data={chartData}
            margin={{
              top: 8,
              right: 12,
              bottom: lineChartXMarginBottom(pointCount),
              left: 8,
            }}
          >
          <XAxis
            type="number"
            dataKey="time"
            domain={["dataMin", "dataMax"]}
            stroke="#64748b"
            tick={xAxisTickStyle(pointCount)}
            minTickGap={xAxisMinTickGap(pointCount)}
            tickFormatter={formatXAxisTickFromTime}
          />
          <YAxis
            type="number"
            stroke="#64748b"
            tick={{ fill: "#94a3b8", fontSize: 11 }}
            tickFormatter={(v) =>
              typeof v === "number" ? formatPriceAxis(v) : String(v)
            }
          />
          <Tooltip content={<PriceChartTooltip />} />
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
          {tradeMarkers.length > 0 && (
            <Scatter
              data={tradeMarkers}
              dataKey="markerY"
              fill="#94a3b8"
              legendType="none"
              isAnimationActive={false}
              shape={(props: {
                cx?: number;
                cy?: number;
                payload?: TradeMarkerPoint;
              }) => {
                const { cx, cy, payload } = props;
                if (cx == null || cy == null || !payload) return null;
                const fill = payload.side === "BUY" ? BUY_FILL : SELL_FILL;
                return (
                  <circle
                    cx={cx}
                    cy={cy}
                    r={3}
                    fill={fill}
                    stroke="#0f172a"
                    strokeWidth={1}
                  />
                );
              }}
            />
          )}
          </ComposedChart>
        </ResponsiveContainer>
      </div>
      {tradeMarkers.length > 0 && (
        <div
          className="mt-2 flex flex-wrap items-center justify-center gap-x-6 gap-y-1 border-t border-slate-800/80 pt-2 text-xs text-slate-400"
          aria-label="Trade marker legend"
        >
          <span className="inline-flex items-center gap-2">
            <span
              className="inline-block h-2.5 w-2.5 shrink-0 rounded-full border border-slate-950"
              style={{ backgroundColor: BUY_FILL }}
              aria-hidden
            />
            <span className="text-slate-300">BUY</span>
          </span>
          <span className="inline-flex items-center gap-2">
            <span
              className="inline-block h-2.5 w-2.5 shrink-0 rounded-full border border-slate-950"
              style={{ backgroundColor: SELL_FILL }}
              aria-hidden
            />
            <span className="text-slate-300">SELL</span>
          </span>
        </div>
      )}
    </div>
  );
}
