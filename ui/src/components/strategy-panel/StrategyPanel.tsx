import { useId } from "react";
import {
  StrategyNumberFields,
  type StrategyNumberFieldConfig,
} from "./StrategyNumberFields.tsx";
import { StrategyParamSection } from "./StrategyParamSection.tsx";
import type { MaParams, RsiParams } from "../../services/api.ts";

const MA_HELP = {
  label: "Moving average parameters" as const,
  text: "Trend-following crossover: the short window averages fewer bars than the long window. The engine buys when the short average crosses above the long, and sells when it crosses below. Smaller windows react faster; larger ones smooth out noise.",
};

const RSI_HELP = {
  label: "RSI parameters" as const,
  text: "Relative Strength Index (0–100) measures recent up vs down moves. This mean-reversion rule buys when RSI falls below oversold and sells when RSI rises above overbought. Period is how many bars are averaged; common defaults are 14 for period and 30 / 70 for thresholds.",
};

type StrategyPanelProps = {
  loading: boolean;
  maParams: MaParams;
  rsiParams: RsiParams;
  onMaShortChange: (n: number) => void;
  onMaLongChange: (n: number) => void;
  onRsiPeriodChange: (n: number) => void;
  onRsiOverboughtChange: (n: number) => void;
  onRsiOversoldChange: (n: number) => void;
};

function maFields(
  maParams: MaParams,
  handlers: {
    onShort: (n: number) => void;
    onLong: (n: number) => void;
  },
): StrategyNumberFieldConfig[] {
  return [
    {
      id: "short",
      label: "Short",
      value: maParams.short,
      min: 1,
      emptyFallback: 1,
      onChange: handlers.onShort,
    },
    {
      id: "long",
      label: "Long",
      value: maParams.long,
      min: 2,
      emptyFallback: 2,
      onChange: handlers.onLong,
    },
  ];
}

function rsiFields(
  rsiParams: RsiParams,
  handlers: {
    onPeriod: (n: number) => void;
    onOverbought: (n: number) => void;
    onOversold: (n: number) => void;
  },
): StrategyNumberFieldConfig[] {
  return [
    {
      id: "p",
      label: "P",
      value: rsiParams.period,
      min: 1,
      emptyFallback: 1,
      onChange: handlers.onPeriod,
    },
    {
      id: "ob",
      label: "OB",
      value: rsiParams.overbought,
      min: 1,
      max: 99,
      emptyFallback: 1,
      onChange: handlers.onOverbought,
    },
    {
      id: "os",
      label: "OS",
      value: rsiParams.oversold,
      min: 0,
      max: 98,
      emptyFallback: 0,
      onChange: handlers.onOversold,
    },
  ];
}

export function StrategyPanel({
  loading,
  maParams,
  rsiParams,
  onMaShortChange,
  onMaLongChange,
  onRsiPeriodChange,
  onRsiOverboughtChange,
  onRsiOversoldChange,
}: StrategyPanelProps) {
  const id = useId();

  return (
    <section className="space-y-3" aria-label="Strategy parameters">
      <h2 className="text-sm font-semibold tracking-tight text-slate-200">
        Strategy Settings
      </h2>

      <div className="space-y-3">
        <StrategyParamSection
          title="MA"
          helpLabel={MA_HELP.label}
          helpText={MA_HELP.text}
        >
          <StrategyNumberFields
            fieldIdPrefix={`${id}-ma`}
            loading={loading}
            fields={maFields(maParams, {
              onShort: onMaShortChange,
              onLong: onMaLongChange,
            })}
          />
        </StrategyParamSection>

        <StrategyParamSection
          title="RSI"
          helpLabel={RSI_HELP.label}
          helpText={RSI_HELP.text}
        >
          <StrategyNumberFields
            fieldIdPrefix={`${id}-rsi`}
            loading={loading}
            fields={rsiFields(rsiParams, {
              onPeriod: onRsiPeriodChange,
              onOverbought: onRsiOverboughtChange,
              onOversold: onRsiOversoldChange,
            })}
          />
        </StrategyParamSection>
      </div>
    </section>
  );
}
