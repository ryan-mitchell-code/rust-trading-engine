# Core Principles

UI is display + API wiring:
- No business logic in React (no recomputing Sharpe, max drawdown, etc.)
- Do **not** recompute drawdown from equity in the front-end; use the **`drawdown_curve`** series from the backend JSON
- Runtime data comes from **`POST /run`** (same **`BacktestRun`** shape as `outputs/results.json`)

Backend = source of truth:
- If something is missing → add it in Rust, not React

Keep components dumb + composable:
- Table = display only
- Chart = display only

`src/components/` is grouped by role (shared shell controls stay at the top level):
- **`charts/`** — candlestick / equity / drawdown charts, `ChartSection`, `chartXAxis`
- **`strategy-panel/`** — strategy settings UI (`StrategyPanel`, `StrategyNumberFields`, styles)
- **`results/`** — strategy metric cards and comparison table
- **Root** — `AppHeader`, `HelpHint`, `DashboardViewToggle`

Prefer flat data:
- Avoid deep nesting in JSON
- Makes TS + UI simpler

No global state (yet):
- Use useState + useEffect
- You don’t need Redux/Zustand