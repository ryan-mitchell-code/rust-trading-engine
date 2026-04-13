# Rust Trading Engine (Learning Project)

Hands-on **Rust** backtester plus **React** UI: run strategies on historical bars, compare metrics, and visualize equity and drawdown.

---

## Start here (reading order)

1. **Run the app** — section below (`./scripts/dev.sh` or `npm run dev`).
2. **Product** — [`docs/product/PRD.md`](docs/product/PRD.md) (source of truth: behavior, API, roadmap). Update it when the product changes.
3. **Domain & backtesting** — [`docs/reference/trading-handbook.md`](docs/reference/trading-handbook.md) (from-scratch concepts for this codebase; §8 quick lookup).

**Learnings & other docs** (open when you need them):

| | |
| --- | --- |
| Journal | [`docs/learning/dev-log.md`](docs/learning/dev-log.md) |
| Rust in this repo | [`docs/learning/rust-learning.md`](docs/learning/rust-learning.md) |
| RSI notes | [`docs/learning/rsi-learning-notes.md`](docs/learning/rsi-learning-notes.md) |
| UI conventions | [`docs/reference/ui-rules.md`](docs/reference/ui-rules.md) |
| Contributor / AI rules | [`docs/reference/rules.md`](docs/reference/rules.md) |
| Vision & constraints (not the PRD) | [`docs/project/context.md`](docs/project/context.md) |

---

## Run locally (API + UI)

**Prerequisites:** [Rust](https://rustup.rs/) (stable), **Node 18+** and npm.

From the repository root:

```bash
./scripts/dev.sh
```

or:

```bash
npm run dev
```

This starts the Rust API on **`http://127.0.0.1:3000`** (`POST /run`) and the Vite app (proxies `/run`). Open **`http://localhost:5173`**, pick a dataset, run a backtest. First compile can take ~1 minute.

**Manual (two terminals):**

```bash
cargo run --manifest-path backend/Cargo.toml -- --serve   # API
npm --prefix ui install && npm --prefix ui run dev       # UI
```

**CLI only** (prints table, writes `outputs/results.json` and CSVs):

```bash
cargo run --manifest-path backend/Cargo.toml
```

Add `-v` / `--verbose` for engine logs (CLI only).

**UI without API** (`Run Backtest` fails until the API is up):

```bash
npm --prefix ui install && npm --prefix ui run dev
```

Optional: Vite can serve `outputs/results.json` at `/results.json` in dev (`ui/backtest-results-plugin.ts`). Normal flow is **API-first** via `POST /run`.

**UI-specific details:** [`ui/README.md`](ui/README.md).

---

## Why this exists

- Learn Rust on a real-shaped system.
- Learn **evaluation** (return, drawdown, Sharpe, benchmarks)—not just “signals that look smart.”
- Experiment with AI-assisted development (Cursor).

> This project is about **building tools to evaluate strategies correctly**, not about shipping a profitable bot.

---
