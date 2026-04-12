import type { ReactNode } from "react";
import { cardClass } from "../../constants/layout.ts";

type ChartSectionProps = {
  title: string;
  dataset: string;
  children: ReactNode;
};

export function ChartSection({ title, dataset, children }: ChartSectionProps) {
  return (
    <section className={`space-y-4 ${cardClass}`}>
      <h2 className="text-sm font-semibold tracking-tight text-slate-200">
        {title} — {dataset}
      </h2>
      <div className="pt-2">{children}</div>
    </section>
  );
}
