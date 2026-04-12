import { useLayoutEffect, useMemo, useRef } from "react";
import {
  CandlestickSeries,
  ColorType,
  createChart,
  createSeriesMarkers,
  CrosshairMode,
  type IChartApi,
  type SeriesMarker,
  type Time,
  type UTCTimestamp,
} from "lightweight-charts";
import type { MarketSeries } from "../../types.ts";
import { parseChartTimestamp } from "./chartXAxis.ts";

type CandlestickChartProps = {
  market: MarketSeries;
  /** e.g. `1d` — daily bars use business-day time (recommended by lightweight-charts). */
  interval?: string;
  /**
   * Per trade: `[id, timestamp, side, price, pnl, capital]` (backend `Vec<Vec<String>>`).
   */
  trades?: string[][];
  /** Fired when the chart is created (passes `null` on teardown). Use for `timeScale()` controls. */
  onChartReady?: (chart: IChartApi | null) => void;
};

type OhlcBar = {
  time: Time;
  open: number;
  high: number;
  low: number;
  close: number;
};

function isDailyInterval(interval: string): boolean {
  return (
    interval === "1d" ||
    interval === "3d" ||
    interval === "1w" ||
    /^\d+d$/.test(interval)
  );
}

/** Stable sort key for mixed `Time` shapes (we only use one shape per series). */
function timeSortKey(t: Time): number {
  if (typeof t === "number") return t;
  if (typeof t === "string") {
    const ms = new Date(t).getTime();
    return Number.isFinite(ms) ? ms / 1000 : 0;
  }
  return Date.UTC(t.year, t.month - 1, t.day) / 1000;
}

function toChartTime(d: Date, interval: string): Time {
  if (isDailyInterval(interval)) {
    return {
      year: d.getUTCFullYear(),
      month: d.getUTCMonth() + 1,
      day: d.getUTCDate(),
    };
  }
  return Math.floor(d.getTime() / 1000) as UTCTimestamp;
}

function marketToBars(market: MarketSeries, interval: string): OhlcBar[] {
  const rows: OhlcBar[] = [];
  for (const row of market) {
    if (row.length < 5) continue;
    const [ts, o, h, l, c] = row;
    const open = Number(o);
    const high = Number(h);
    const low = Number(l);
    const close = Number(c);
    if (
      !Number.isFinite(open) ||
      !Number.isFinite(high) ||
      !Number.isFinite(low) ||
      !Number.isFinite(close)
    ) {
      continue;
    }
    const d = parseChartTimestamp(typeof ts === "string" ? ts : String(ts));
    if (!d) continue;
    rows.push({
      time: toChartTime(d, interval),
      open,
      high,
      low,
      close,
    });
  }
  rows.sort((a, b) => timeSortKey(a.time) - timeSortKey(b.time));
  return rows;
}

/** Preset windows use the last *N* daily bars (approx. calendar span for `1d` crypto data). */
export type ChartRangePreset = "all" | "1m" | "3m" | "6m" | "1y";

const PRESET_BAR_SPAN: Record<Exclude<ChartRangePreset, "all">, number> = {
  "1m": 30,
  "3m": 90,
  "6m": 180,
  "1y": 365,
};

/**
 * Adjusts the visible range using the same `Time` values as the candle series
 * (`BusinessDay` for daily intervals, `UTCTimestamp` otherwise).
 */
export function applyChartRangePreset(
  chart: IChartApi,
  market: MarketSeries,
  interval: string,
  preset: ChartRangePreset,
): void {
  const bars = marketToBars(market, interval);
  if (bars.length === 0) return;

  if (preset === "all") {
    chart.timeScale().fitContent();
    return;
  }

  const n = PRESET_BAR_SPAN[preset];
  const fromIdx = Math.max(0, bars.length - n);
  const from = bars[fromIdx]!.time;
  const to = bars[bars.length - 1]!.time;
  chart.timeScale().setVisibleRange({ from, to });
}

/** OHLC bar direction (close vs open) — shared by series styling and legend. */
export const CANDLE_UP_COLOR = "#22c55e";
export const CANDLE_DOWN_COLOR = "#ef4444";

/**
 * Executed trades — hues chosen to avoid clashing with candle green/red.
 */
export const TRADE_BUY_MARKER_COLOR = "#38bdf8";
export const TRADE_SELL_MARKER_COLOR = "#f59e0b";

/**
 * Map backend trade rows to lightweight-charts series markers, using the same time
 * scale as {@link marketToBars} (`toChartTime` + `interval`).
 */
export function buildMarkers(
  trades: string[][],
  interval: string,
): SeriesMarker<Time>[] {
  const out: SeriesMarker<Time>[] = [];
  for (const row of trades) {
    if (row.length < 3) continue;
    const ts = row[1];
    const sideRaw = String(row[2]).trim().toUpperCase();
    const d = parseChartTimestamp(typeof ts === "string" ? ts : String(ts));
    if (!d) continue;
    const time = toChartTime(d, interval);
    if (sideRaw === "BUY") {
      out.push({
        time,
        position: "belowBar",
        shape: "arrowUp",
        color: TRADE_BUY_MARKER_COLOR,
      });
    } else if (sideRaw === "SELL") {
      out.push({
        time,
        position: "aboveBar",
        shape: "arrowDown",
        color: TRADE_SELL_MARKER_COLOR,
      });
    }
  }
  out.sort((a, b) => timeSortKey(a.time) - timeSortKey(b.time));
  return out;
}

const CHART_MIN_WIDTH = 280;
const CHART_MIN_HEIGHT = 320;

export function CandlestickChart({
  market,
  interval = "1d",
  trades,
  onChartReady,
}: CandlestickChartProps) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const bars = useMemo(() => marketToBars(market, interval), [market, interval]);
  const markers = useMemo(
    () => buildMarkers(trades ?? [], interval),
    [trades, interval],
  );

  useLayoutEffect(() => {
    const el = containerRef.current;
    if (!el) return;

    const data = bars;
    if (data.length === 0) return;

    const width = Math.max(el.clientWidth, CHART_MIN_WIDTH);
    const height = Math.max(el.clientHeight, CHART_MIN_HEIGHT);

    const chart = createChart(el, {
      width,
      height,
      layout: {
        background: { type: ColorType.Solid, color: "#0f172a" },
        textColor: "#94a3b8",
      },
      grid: {
        vertLines: { color: "#1e293b" },
        horzLines: { color: "#1e293b" },
      },
      crosshair: {
        mode: CrosshairMode.Normal,
        vertLine: {
          color: "#64748b",
          labelBackgroundColor: "#334155",
        },
        horzLine: {
          color: "#64748b",
          labelBackgroundColor: "#334155",
        },
      },
      rightPriceScale: {
        borderColor: "#334155",
        scaleMargins: {
          top: 0.08,
          bottom: 0.12,
        },
      },
      timeScale: {
        borderColor: "#334155",
        fixLeftEdge: false,
        fixRightEdge: false,
      },
    });

    const series = chart.addSeries(CandlestickSeries, {
      upColor: CANDLE_UP_COLOR,
      downColor: CANDLE_DOWN_COLOR,
      borderVisible: false,
      wickUpColor: CANDLE_UP_COLOR,
      wickDownColor: CANDLE_DOWN_COLOR,
      priceFormat: {
        type: "price",
        minMove: 0.01,
        precision: 2,
      },
    });
    series.setData(data);
    createSeriesMarkers(series, markers);
    chart.timeScale().fitContent();
    onChartReady?.(chart);

    const ro = new ResizeObserver(() => {
      if (!containerRef.current) return;
      chart.applyOptions({
        width: Math.max(containerRef.current.clientWidth, CHART_MIN_WIDTH),
        height: Math.max(containerRef.current.clientHeight, CHART_MIN_HEIGHT),
      });
    });
    ro.observe(el);

    return () => {
      ro.disconnect();
      onChartReady?.(null);
      chart.remove();
    };
  }, [bars, markers, onChartReady]);

  if (market.length === 0) {
    return (
      <p className="text-sm text-slate-500">No market data to plot.</p>
    );
  }

  if (bars.length === 0) {
    return (
      <p className="text-sm text-slate-500">
        No valid OHLC bars to plot (check timestamps).
      </p>
    );
  }

  return (
    <div className="space-y-2">
      <div
        className="flex flex-wrap items-baseline gap-x-3 gap-y-2 border-b border-slate-800/80 pb-2 text-[11px] leading-tight text-slate-500 sm:text-xs"
        role="group"
        aria-label="Chart legend"
      >
        <span className="font-medium text-slate-400">Legend</span>
        <span className="text-slate-600" aria-hidden>
          |
        </span>
        <span className="font-medium text-slate-500">Candles (price)</span>
        <span className="inline-flex items-center gap-1.5">
          <span
            className="h-2.5 w-3 shrink-0 rounded-sm"
            style={{ backgroundColor: CANDLE_UP_COLOR }}
            aria-hidden
          />
          <span className="text-slate-400">Up bar (close ≥ open)</span>
        </span>
        <span className="inline-flex items-center gap-1.5">
          <span
            className="h-2.5 w-3 shrink-0 rounded-sm"
            style={{ backgroundColor: CANDLE_DOWN_COLOR }}
            aria-hidden
          />
          <span className="text-slate-400">Down bar (close &lt; open)</span>
        </span>
        <span className="text-slate-600" aria-hidden>
          |
        </span>
        <span className="font-medium text-slate-500">Trades (strategy)</span>
        <span className="inline-flex items-center gap-1">
          <span
            className="text-sm leading-none"
            style={{ color: TRADE_BUY_MARKER_COLOR }}
            aria-hidden
          >
            ▲
          </span>
          <span className="text-slate-400">Buy</span>
        </span>
        <span className="inline-flex items-center gap-1">
          <span
            className="text-sm leading-none"
            style={{ color: TRADE_SELL_MARKER_COLOR }}
            aria-hidden
          >
            ▼
          </span>
          <span className="text-slate-400">Sell</span>
        </span>
        {trades === undefined && (
          <span className="text-slate-600 italic">
            Select a strategy to show trade markers.
          </span>
        )}
        {trades !== undefined && markers.length === 0 && (
          <span className="text-slate-600 italic">No trades in this run.</span>
        )}
      </div>
      <div
        className="h-80 w-full min-h-[320px] min-w-0 rounded-lg border border-slate-800 bg-slate-900/40 p-1"
        ref={containerRef}
      />
    </div>
  );
}
