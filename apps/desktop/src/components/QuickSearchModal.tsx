import { useState } from "react";
import { Search, X } from "lucide-react";
import { useShowStore } from "../stores/useShowStore";

export function QuickSearchModal({ open, onClose }: { open: boolean; onClose: () => void }) {
  const [q, setQ] = useState("");
  const { playlist, triggerSlideLive, setWorkspaceTab } = useShowStore();

  if (!open) return null;

  const allSlides = playlist.flatMap((p) =>
    p.slides.map((s) => ({ ...s, playlistTitle: p.title }))
  );
  const filtered = allSlides.filter(
    (s) =>
      s.text.toLowerCase().includes(q.toLowerCase()) ||
      s.label.toLowerCase().includes(q.toLowerCase()) ||
      s.playlistTitle.toLowerCase().includes(q.toLowerCase())
  );

  return (
    <div className="fixed inset-0 z-50 bg-black/70 flex items-start justify-center pt-24">
      <div className="w-full max-w-lg bg-zinc-900 border border-zinc-700 rounded-lg shadow-2xl overflow-hidden">
        <div className="flex items-center gap-2 px-3 py-2.5 border-b border-zinc-800">
          <Search className="w-4 h-4 text-zinc-400" />
          <input
            autoFocus
            type="text"
            value={q}
            onChange={(e) => setQ(e.target.value)}
            placeholder="Quick Search slides, songs, scriptures..."
            className="flex-1 bg-transparent text-sm text-zinc-100 focus:outline-none"
          />
          <button onClick={onClose} className="text-zinc-500 hover:text-zinc-200">
            <X className="w-4 h-4" />
          </button>
        </div>
        <div className="max-h-72 overflow-y-auto p-1.5 space-y-1">
          {filtered.slice(0, 12).map((s) => (
            <button
              key={s.id}
              onClick={() => {
                triggerSlideLive(s.id);
                setWorkspaceTab("Show");
                onClose();
              }}
              className="w-full text-left px-3 py-2 rounded hover:bg-zinc-800 flex items-center justify-between gap-2"
            >
              <div className="truncate">
                <div className="text-xs font-medium text-zinc-200 truncate">{s.text}</div>
                <div className="text-[10px] text-zinc-500">{s.playlistTitle} • {s.label}</div>
              </div>
              <span className="text-[10px] px-1.5 py-0.5 rounded bg-red-600/20 text-red-300">
                Go Live
              </span>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
