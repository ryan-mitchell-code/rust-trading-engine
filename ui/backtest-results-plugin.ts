import fs from "node:fs";
import type { IncomingMessage, ServerResponse } from "node:http";
import path from "node:path";
import type { Plugin } from "vite";
import { fileURLToPath } from "node:url";

const uiDir = path.dirname(fileURLToPath(import.meta.url));

function resultsFilePath(): string {
  return path.resolve(uiDir, "../outputs/results.json");
}

function serveResultsJson(_req: IncomingMessage, res: ServerResponse) {
  const file = resultsFilePath();
  if (!fs.existsSync(file)) {
    res.statusCode = 404;
    res.setHeader("Content-Type", "application/json; charset=utf-8");
    res.end(
      JSON.stringify({
        error:
          "Missing outputs/results.json. Run: cargo run --manifest-path backend/Cargo.toml",
      }),
    );
    return;
  }
  res.setHeader("Content-Type", "application/json; charset=utf-8");
  fs.createReadStream(file).pipe(res);
}

/** Serves workspace `outputs/results.json` at `/results.json` for dev and preview. */
export function backtestResultsPlugin(): Plugin {
  return {
    name: "backtest-results",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (req.url === "/results.json" || req.url?.startsWith("/results.json?")) {
          serveResultsJson(req, res);
          return;
        }
        next();
      });
    },
    configurePreviewServer(server) {
      server.middlewares.use((req, res, next) => {
        if (req.url === "/results.json" || req.url?.startsWith("/results.json?")) {
          serveResultsJson(req, res);
          return;
        }
        next();
      });
    },
  };
}
