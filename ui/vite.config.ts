import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { backtestResultsPlugin } from "./backtest-results-plugin.ts";

export default defineConfig({
  plugins: [react(), tailwindcss(), backtestResultsPlugin()],
});
