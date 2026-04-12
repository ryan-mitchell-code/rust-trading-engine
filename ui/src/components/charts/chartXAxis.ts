/**
 * Parse `timestamp` from backend: ISO 8601 (e.g. Binance), or legacy cache as Unix ms string.
 */
export function parseChartTimestamp(raw: string): Date | null {
  const trimmed = raw.trim();
  if (/^\d{10,}$/.test(trimmed)) {
    const ms = Number(trimmed);
    if (Number.isFinite(ms)) return new Date(ms);
  }
  const d = new Date(trimmed);
  const t = d.getTime();
  return Number.isNaN(t) ? null : d;
}

/** Compact date for x-axis (includes year when the span can cross years). */
export function formatChartXAxisTickLabel(raw: string): string {
  const d = parseChartTimestamp(raw);
  if (!d) return raw.length > 12 ? `${raw.slice(0, 10)}…` : raw;
  return d.toLocaleDateString(undefined, {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

export function formatChartTooltipTimestamp(raw: unknown): string {
  const s = String(raw);
  const d = parseChartTimestamp(s);
  if (!d) return s;
  return d.toLocaleString(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  });
}

/**
 * Long series (e.g. ~1000 daily bars) need fewer ticks, but ticks should never be fully hidden.
 */
export function xAxisMinTickGap(pointCount: number): number {
  if (pointCount > 700) return 88;
  if (pointCount > 400) return 64;
  if (pointCount > 120) return 44;
  if (pointCount > 80) return 36;
  return 28;
}

export function xAxisTickStyle(pointCount: number): {
  fill: string;
  fontSize: number;
} {
  return {
    fill: "#94a3b8",
    fontSize: pointCount > 500 ? 9 : 10,
  };
}

/** Extra bottom space when x-axis has date labels. */
export function lineChartXMarginBottom(pointCount: number): number {
  const base = pointCount > 80 ? 22 : 16;
  return base + 8 + (pointCount > 400 ? 4 : 0);
}
