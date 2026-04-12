# rust-trader UI

React + Vite front-end for the backtest dashboard. The normal flow calls **`POST /run`** on the Rust API and renders the returned **`BacktestRun`** JSON. See [`docs/ui-rules.md`](../docs/ui-rules.md).

## Prerequisites

- Node **18+**
- The **backend API** running on **`http://127.0.0.1:3000`** (Vite proxies `/run` there — see `vite.config.ts`)

## Run (recommended)

From the **repository root**, start the API and this dev server together:

```bash
./scripts/dev.sh
```

Or:

```bash
npm run dev
```

Open the URL Vite prints (usually **`http://localhost:5173`**), then use **Run Backtest** in the app.

## Run (UI only)

If you already started the API elsewhere (`cargo run --manifest-path backend/Cargo.toml -- --serve`):

```bash
npm install
npm run dev
```

## Optional: static `results.json`

The Vite dev server can also serve **`../outputs/results.json`** at **`/results.json`** via `backtest-results-plugin.ts` (useful for quick inspection without hitting `POST /run`). If that file is missing, the plugin returns a JSON error with a hint.

## Build

```bash
npm run build
npm run preview   # optional: production build + local preview
```

## Stack

- React 19, TypeScript, Tailwind CSS 4, Recharts
