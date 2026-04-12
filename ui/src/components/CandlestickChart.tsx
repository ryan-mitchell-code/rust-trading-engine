import { useLayoutEffect, useMemo, useRef } from "react";
import {
  CandlestickSeries,
  ColorType,
  createChart,
  CrosshairMode,
  type Time,
  type UTCTimestamp,
} from "lightweight-charts";
import type { MarketSeries } from "../types.ts";
import { parseChartTimestamp } from "./chartXAxis.ts";

type CandlestickChartProps = {
  market: MarketSeries;
  /** e.g. `1d` — daily bars use business-day time (recommended by lightweight-charts). */
  interval?: string;
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

const CHART_MIN_WIDTH = 280;
const CHART_MIN_HEIGHT = 320;

export function CandlestickChart({
  market,
  interval = "1d",
}: CandlestickChartProps) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const bars = useMemo(() => marketToBars(market, interval), [market, interval]);

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
      upColor: "#22c55e",
      downColor: "#ef4444",
      borderVisible: false,
      wickUpColor: "#22c55e",
      wickDownColor: "#ef4444",
      priceFormat: {
        type: "price",
        minMove: 0.01,
        precision: 2,
      },
    });
    series.setData(data);
    chart.timeScale().fitContent();

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
      chart.remove();
    };
  }, [bars]);

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
    <div
      className="h-80 w-full min-h-[320px] min-w-0 rounded-lg border border-slate-800 bg-slate-900/40 p-1"
      ref={containerRef}
    />
  );
}
