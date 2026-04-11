# Core Principles

UI is read-only:
- No business logic in React
- No recalculating metrics
- Everything comes from backend JSON

Backend = source of truth:
- If something is missing → add it in Rust, not React

Keep components dumb + composable:
- Table = display only
- Chart = display only

Prefer flat data:
- Avoid deep nesting in JSON
- Makes TS + UI simpler

No global state (yet):
- Use useState + useEffect
- You don’t need Redux/Zustand