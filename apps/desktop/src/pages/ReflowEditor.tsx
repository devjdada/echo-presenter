import { useState } from "react";
import { useShowStore } from "../stores/useShowStore";

export function ReflowEditor() {
  const { playlist, activePlaylistId, triggerSlideLive } = useShowStore();
  const currentItem = playlist.find((p) => p.id === activePlaylistId) ?? playlist[0];

  const initialDoc = (currentItem?.slides ?? [])
    .map((s) => `[${s.label}]\n${s.text}`)
    .join("\n\n");
  const [docText, setDocText] = useState(initialDoc);

  const parsedSlides = docText
    .split(/\n{2,}/)
    .map((block) => block.trim())
    .filter(Boolean)
    .map((block, idx) => {
      const lines = block.split("\n");
      const first = lines[0] ?? "";
      const hasHeader = first.startsWith("[") && first.endsWith("]");
      const label = hasHeader ? first.slice(1, -1) : `Slide ${idx + 1}`;
      const text = hasHeader ? lines.slice(1).join("\n") : block;
      return { id: `reflow-${idx + 1}`, label, text };
    });

  return (
    <div className="flex-1 min-h-0 flex bg-zinc-950 text-zinc-100">
      <div className="w-1/2 border-r border-zinc-800 flex flex-col p-4 gap-2">
        <div className="flex items-center justify-between">
          <span className="text-xs font-semibold uppercase tracking-wider text-zinc-400">
            Reflow Document — {currentItem?.title}
          </span>
          <span className="text-[11px] text-zinc-500">Blank line = new slide</span>
        </div>
        <textarea
          value={docText}
          onChange={(e) => setDocText(e.target.value)}
          className="flex-1 bg-zinc-900 border border-zinc-800 rounded p-3 text-sm font-mono text-zinc-200 focus:outline-none focus:border-indigo-500 resize-none leading-relaxed"
        />
      </div>

      <div className="w-1/2 p-4 overflow-y-auto">
        <div className="text-xs font-semibold uppercase tracking-wider text-zinc-400 mb-3">
          Live Slide Preview ({parsedSlides.length} slides)
        </div>
        <div className="grid grid-cols-2 gap-3">
          {parsedSlides.map((s, idx) => (
            <button
              key={s.id}
              onClick={() => {
                const orig = currentItem?.slides[idx];
                if (orig) triggerSlideLive(orig.id);
              }}
              className="text-left rounded border border-zinc-800 hover:border-indigo-500 bg-zinc-900 overflow-hidden"
            >
              <div className="aspect-video bg-gradient-to-br from-slate-900 via-blue-950 to-zinc-950 p-3 flex items-center justify-center text-center">
                <p className="text-xs text-white whitespace-pre-line line-clamp-4">{s.text}</p>
              </div>
              <div className="px-2 py-1 text-[11px] text-zinc-400 flex justify-between border-t border-zinc-800">
                <span>{s.label}</span>
                <span>{idx + 1}</span>
              </div>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
