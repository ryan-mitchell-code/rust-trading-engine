# rust-trader UI

React + Vite front-end for **`outputs/results.json`**. It is read-only: charts and tables display what the Rust backtester exported; see [`docs/ui-rules.md`](../docs/ui-rules.md).

## Prerequisites

- Node **18+**
- A generated **`../outputs/results.json`** (run the backend from the repo root)

## Run

From this directory:

```bash
npm install
npm run dev
```

Open the URL Vite prints (usually `http://localhost:5173`). The dev server serves **`/results.json`** from the workspace file `outputs/results.json` via `backtest-results-plugin.ts`. If the file is missing, the endpoint returns JSON with an error and a hint to run the backend.

## Build

```bash
npm run build
npm run preview   # optional: production build + local preview (still serves results.json the same way)
```

## Stack

- React 19, TypeScript, Tailwind CSS 4, Recharts
