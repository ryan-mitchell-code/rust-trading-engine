import type { ReactNode } from "react";
import { HelpHint } from "../HelpHint.tsx";
import {
  strategyPanelSectionClass,
  strategySectionTitleClass,
} from "./strategyPanelStyles.ts";

type StrategyParamSectionProps = {
  title: string;
  helpLabel: string;
  helpText: string;
  children: ReactNode;
};

export function StrategyParamSection({
  title,
  helpLabel,
  helpText,
  children,
}: StrategyParamSectionProps) {
  return (
    <div className={strategyPanelSectionClass}>
      <div className="mb-2 flex items-center justify-between gap-3">
        <span className={strategySectionTitleClass}>{title}</span>
        <HelpHint label={helpLabel} text={helpText} />
      </div>
      {children}
    </div>
  );
}
