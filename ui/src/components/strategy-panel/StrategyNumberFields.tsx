import {
  strategyFieldLabelClass,
  strategyNumberInputClass,
} from "./strategyPanelStyles.ts";

export type StrategyNumberFieldConfig = {
  /** Suffix for stable ids: `${fieldIdPrefix}-${id}` */
  id: string;
  label: string;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  /** Used when the input is empty or not a valid integer before applying min/max */
  emptyFallback: number;
  onChange: (n: number) => void;
};

type StrategyNumberFieldsProps = {
  fieldIdPrefix: string;
  loading: boolean;
  fields: StrategyNumberFieldConfig[];
};

function parseAndClamp(
  raw: string,
  emptyFallback: number,
  min?: number,
  max?: number,
): number {
  const parsed = parseInt(raw, 10);
  let n = Number.isNaN(parsed) ? emptyFallback : parsed;
  if (min !== undefined) n = Math.max(min, n);
  if (max !== undefined) n = Math.min(max, n);
  return n;
}

export function StrategyNumberFields({
  fieldIdPrefix,
  loading,
  fields,
}: StrategyNumberFieldsProps) {
  return (
    <div className="flex flex-wrap items-center gap-3">
      {fields.map((field) => (
        <label
          key={field.id}
          htmlFor={`${fieldIdPrefix}-${field.id}`}
          className="flex items-center gap-1.5"
        >
          <span className={strategyFieldLabelClass}>{field.label}</span>
          <input
            id={`${fieldIdPrefix}-${field.id}`}
            type="number"
            min={field.min}
            max={field.max}
            step={field.step ?? 1}
            value={field.value}
            onChange={(e) =>
              field.onChange(
                parseAndClamp(
                  e.target.value,
                  field.emptyFallback,
                  field.min,
                  field.max,
                ),
              )
            }
            disabled={loading}
            className={strategyNumberInputClass}
          />
        </label>
      ))}
    </div>
  );
}
