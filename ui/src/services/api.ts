import type { BacktestRun } from "../types.ts";

export type Dataset = "BTCUSDT" | "ETHUSDT";

const INTERVAL = "1d" as const;

export async function fetchBacktestRun(dataset: Dataset): Promise<BacktestRun> {
  const res = await fetch("/run", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ dataset, interval: INTERVAL }),
  });
  if (!res.ok) {
    throw new Error(`${res.status} ${res.statusText}`);
  }
  return res.json() as Promise<BacktestRun>;
}
