#!/usr/bin/env sh
# Run a full backtest, then start the Vite dev server (reads outputs/results.json).
# Usage: ./scripts/dev.sh   or   ./scripts/dev.sh -v
# Requires: Rust toolchain, Node 18+, npm. First run installs ui deps via npm --prefix ui.

set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT" || exit 1

echo "==> Backtest (cargo run --manifest-path backend/Cargo.toml)"
cargo run --manifest-path backend/Cargo.toml "$@"

echo ""
echo "==> UI (npm --prefix ui run dev)"
if [ ! -d ui/node_modules ]; then
  echo "First run: installing npm dependencies..."
  npm --prefix ui install
fi
exec npm --prefix ui run dev
