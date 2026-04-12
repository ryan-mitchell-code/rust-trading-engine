import type { BacktestRun } from "../types.ts";

export type Dataset = "BTCUSDT" | "ETHUSDT";

/** Posted to `POST /run` — candlestick chart uses this for time scale (e.g. business days for `1d`). */
export const BACKTEST_INTERVAL = "1d" as const;

async function messageFromFailedResponse(res: Response): Promise<string> {
  const fallback = `${res.status} ${res.statusText}`;
  let text: string;
  try {
    text = await res.text();
  } catch {
    return fallback;
  }
  const trimmed = text.trim();
  if (!trimmed) return fallback;
  try {
    const data = JSON.parse(trimmed) as { error?: unknown };
    if (typeof data.error === "string" && data.error.trim()) {
      return data.error.trim();
    }
  } catch {
    return trimmed.length > 500 ? `${trimmed.slice(0, 500)}…` : trimmed;
  }
  return fallback;
}

export type MaParams = { short: number; long: number };

export async function fetchBacktestRun(
  dataset: Dataset,
  ma: MaParams,
): Promise<BacktestRun> {
  const res = await fetch("/run", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      dataset,
      interval: BACKTEST_INTERVAL,
      ma_short: ma.short,
      ma_long: ma.long,
    }),
  });
  if (!res.ok) {
    throw new Error(await messageFromFailedResponse(res));
  }
  return res.json() as Promise<BacktestRun>;
}
