#!/usr/bin/env sh
# Start the HTTP API (POST /run on port 3000), then the Vite dev server.
# Use "Run Backtest" in the UI to load data — requests go to the API (not static results.json).
#
# Usage: ./scripts/dev.sh
# Requires: Rust toolchain, Node 18+, npm. First run installs ui deps via npm --prefix ui.
#
# One-shot CLI backtest (writes outputs/, no API): cargo run --manifest-path backend/Cargo.toml

set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT" || exit 1

BACKEND_PID=""
cleanup() {
  if [ -n "${BACKEND_PID:-}" ]; then
    if kill -0 "$BACKEND_PID" 2>/dev/null; then
      kill "$BACKEND_PID" 2>/dev/null || true
      wait "$BACKEND_PID" 2>/dev/null || true
    fi
  fi
}
# INT: Ctrl+C · TERM: graceful stop · HUP: closing the terminal tab/window · EXIT: npm exited
trap cleanup INT TERM HUP EXIT

echo "==> Backend API — http://127.0.0.1:3000 (POST /run)"
echo "    (first compile can take a minute)"
cargo run --manifest-path backend/Cargo.toml -- --serve &
BACKEND_PID=$!

echo "    Waiting for port 3000..."
if command -v nc >/dev/null 2>&1; then
  i=0
  while [ "$i" -lt 600 ]; do
    if nc -z 127.0.0.1 3000 2>/dev/null; then
      echo "    Listening."
      break
    fi
    i=$((i + 1))
    sleep 0.1
  done
  if ! nc -z 127.0.0.1 3000 2>/dev/null; then
    echo "    Warning: port 3000 not open in time; check backend logs above." >&2
  fi
else
  sleep 3
fi

echo ""
echo "==> UI (Vite — http://localhost:5173, proxies /run → API)"
if [ ! -d ui/node_modules ]; then
  echo "First run: installing npm dependencies..."
  npm --prefix ui install
fi

npm --prefix ui run dev
