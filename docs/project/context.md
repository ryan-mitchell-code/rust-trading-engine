# Project context (vision & constraints)

**Product facts** (what ships, API shape, roadmap) live in **`docs/product/PRD.md`**. Update that file when behavior or scope changes.

This page only captures **motivation and guardrails** that are easy to lose if we duplicated the PRD.

---

## Long-term direction

Evolve toward a **strategy simulation platform**: multiple algorithms (“bots”) compared on shared data, with clear capital and state per strategy. Today the arena runs a **fixed** strategy set; configurability is on the roadmap in the PRD.

---

## AI-assisted development

Cursor assists implementation and explanation; the human owns architecture and intent. Prefer **simple, explainable** code over clever abstractions.

---

## Constraints

- No external **trading** libraries (HTTP for market data is fine).
- Focus on **learning**, not live profitability.
- Idiomatic Rust where it helps clarity.
- Avoid premature abstraction.

---

## Documentation habits

- **PRD** = product source of truth.
- **Trading handbook** = teach domain language and tie terms to this repo.
- **Dev log** = narrative history.
- Documentation links are listed in the repository root **`README.md`**.
