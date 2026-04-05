# Project Context

This is a Rust-based trading backtesting engine built as a learning project.

The long-term goal is to evolve this into a **strategy simulation platform** where multiple trading algorithms ("bots") can be compared and evaluated visually.

---

## 🎯 Goals

- Learn Rust through a real-world system
- Understand trading strategy design and risk
- Build a modular and extensible simulation engine
- Enable comparison of multiple trading strategies

---

## 📊 Simulation Goal

The system should support:

- Running trading strategies on historical data
- Tracking capital over time
- Producing outputs suitable for visualisation
- Comparing performance across strategies

A “bot” will eventually represent:

- A strategy
- Capital
- Internal state

---

## 📈 Visualisation Goal

Visual feedback is essential for learning.

The system should produce:

- Equity curve (capital over time)
- Trade log (buy/sell events and outcomes)

These outputs should be easy to:

- Open in Excel / Google Sheets
- Later integrate into a React frontend

---

## 🤖 AI-Assisted Development

Cursor is used as a development assistant.

Guidelines:

- The developer (human) defines architecture and intent
- Cursor assists with implementation and explanations
- Code must remain understandable and explainable
- Avoid over-engineered or overly complex solutions

---

## 📚 Documentation Goals

This project documents both implementation and learning.

### Objectives

- Capture key Rust concepts as they are introduced
- Explain trading concepts in simple terms
- Maintain a glossary of domain-specific terminology
- Show progression from simple to more advanced systems

---

### Approach

- Keep documentation concise and practical
- Tie explanations directly to code usage
- Prefer clarity over completeness
- Avoid duplicating information unnecessarily

---

### Files

- `/docs/rust-learning.md` → Rust concepts and learnings
- `/docs/dev-log.md` → Progress, decisions, mistakes
- `/docs/glossary.md` → Trading concepts
- `README.md` → High-level overview

---

## ⚙️ Current Features

- CSV data loading
- Moving average crossover strategy
- Position tracking
- Basic profit calculation

---

## 🧱 Design Principles

- Keep code simple and readable
- Prefer explicit state over hidden logic
- Avoid premature optimisation
- Build incrementally with clear feedback loops

---

## ⚠️ Constraints

- No external trading libraries
- Focus on learning, not profitability
- Code should be idiomatic Rust where possible
- Avoid unnecessary abstraction early

---

## 🚀 Next Steps (Current Focus)

- Add capital tracking
- Implement position sizing
- Track equity over time
- Export equity curve to CSV
- Export trade log for analysis
