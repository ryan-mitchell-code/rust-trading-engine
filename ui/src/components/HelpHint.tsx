import { useId } from "react";

type HelpHintProps = {
  /** Shown in the tooltip panel */
  text: string;
  /** Accessible name for the help control */
  label: string;
};

export function HelpHint({ text, label }: HelpHintProps) {
  const tipId = useId();
  return (
    <span className="group relative inline-flex shrink-0">
      <button
        type="button"
        className="inline-flex h-5 min-w-5 items-center justify-center rounded-full border border-slate-600 bg-slate-800/90 px-1 text-[10px] font-semibold leading-none text-slate-400 transition hover:border-slate-500 hover:text-slate-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-500/60"
        aria-label={label}
        aria-describedby={tipId}
      >
        ?
      </button>
      <span
        id={tipId}
        role="tooltip"
        className="pointer-events-none invisible absolute left-1/2 top-full z-40 mt-2 w-64 max-w-[min(18rem,calc(100vw-2rem))] -translate-x-1/2 rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-left text-xs leading-relaxed text-slate-300 opacity-0 shadow-xl transition-opacity duration-150 group-hover:visible group-hover:opacity-100 group-focus-within:visible group-focus-within:opacity-100"
      >
        {text}
      </span>
    </span>
  );
}
