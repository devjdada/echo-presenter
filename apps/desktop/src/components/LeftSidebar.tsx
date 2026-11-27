import { invoke } from "@tauri-apps/api/core";
import { Folder, ListMusic, Music, BookOpen, Megaphone, Plus, FileUp, Download } from "lucide-react";
import { useShowStore } from "../stores/useShowStore";

const isTauri = () => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export function LeftSidebar() {
  const { libraries, playlist, activePlaylistId, selectPlaylistItem, addGeneratedPlaylist } =
    useShowStore();

  const iconForKind = (kind: string) => {
    switch (kind) {
      case "song":
        return <Music className="w-3.5 h-3.5 text-sky-400 shrink-0" />;
      case "sermon":
        return <BookOpen className="w-3.5 h-3.5 text-amber-400 shrink-0" />;
      default:
        return <Megaphone className="w-3.5 h-3.5 text-rose-400 shrink-0" />;
    }
  };

  return (
    <aside className="w-60 bg-zinc-900/90 border-r border-zinc-800 flex flex-col shrink-0 select-none">
      {/* Top: Libraries */}
      <div className="flex-1 min-h-0 flex flex-col border-b border-zinc-800">
        <div className="h-8 px-3 flex items-center justify-between border-b border-zinc-800/80 bg-zinc-950/40">
          <span className="text-[11px] font-semibold uppercase tracking-wider text-zinc-400">
            Libraries
          </span>
          <button
            type="button"
            className="text-zinc-500 hover:text-zinc-200 p-0.5 rounded"
            title="New Library Item"
          >
            <Plus className="w-3.5 h-3.5" />
          </button>
        </div>
        <div className="flex-1 overflow-y-auto p-1.5 space-y-1">
          {libraries.map((lib) => (
            <div key={lib.id} className="space-y-0.5">
              <div className="flex items-center gap-1.5 px-2 py-1 text-xs font-medium text-zinc-300">
                <Folder className="w-3.5 h-3.5 text-zinc-500" />
                <span>{lib.name}</span>
              </div>
              <div className="pl-5 space-y-0.5">
                {lib.items.map((item) => (
                  <button
                    key={item}
                    type="button"
                    className="w-full text-left px-2 py-1 rounded text-xs text-zinc-400 hover:text-zinc-100 hover:bg-zinc-800/70 truncate"
                  >
                    {item}
                  </button>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Bottom: Service Playlist */}
      <div className="flex-1 min-h-0 flex flex-col">
        <div className="h-8 px-3 flex items-center justify-between border-b border-zinc-800/80 bg-zinc-950/40">
          <span className="flex items-center gap-1.5 text-[11px] font-semibold uppercase tracking-wider text-zinc-400">
            <ListMusic className="w-3.5 h-3.5 text-indigo-400" />
            Sunday Service
          </span>
          <div className="flex items-center gap-1">
            <button
              type="button"
              onClick={async () => {
                if (!isTauri()) return;
                const path = window.prompt("Enter .pptx file path:");
                if (!path) return;
                const slides = await invoke<{ id: string; label: string; text: string }[]>("import_pptx", {
                  filePath: path,
                });
                await addGeneratedPlaylist(
                  "Imported PowerPoint",
                  slides.map((s) => ({
                    id: s.id,
                    label: s.label,
                    group: "announcement",
                    text: s.text,
                    bgGradient: "from-slate-900 via-indigo-950 to-zinc-950",
                  }))
                );
              }}
              className="text-zinc-500 hover:text-zinc-200 p-0.5 rounded"
              title="Import PowerPoint (.pptx)"
            >
              <FileUp className="w-3.5 h-3.5" />
            </button>
            <button
              type="button"
              onClick={async () => {
                if (!isTauri()) return;
                const path = window.prompt("Export .echopres path:", "service.echopres");
                if (path) {
                  await invoke("export_echopres", {
                    filePath: path,
                    playlistsJson: JSON.stringify(playlist),
                  });
                }
              }}
              className="text-zinc-500 hover:text-zinc-200 p-0.5 rounded"
              title="Export .echopres"
            >
              <Download className="w-3.5 h-3.5" />
            </button>
            <button
              type="button"
              onClick={async () => {
                if (!isTauri()) return;
                const path = window.prompt("Import .echopres path:");
                if (path) {
                  await invoke("import_echopres", { filePath: path });
                }
              }}
              className="text-zinc-500 hover:text-zinc-200 p-0.5 rounded"
              title="Import .echopres"
            >
              <Plus className="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
        <div className="flex-1 overflow-y-auto p-1.5 space-y-1">
          {playlist.map((item) => {
            const active = item.id === activePlaylistId;
            return (
              <button
                key={item.id}
                onClick={() => selectPlaylistItem(item.id)}
                className={`w-full flex items-center gap-2 px-2.5 py-2 rounded text-left text-xs transition-colors ${
                  active
                    ? "bg-indigo-600/20 border border-indigo-500/40 text-zinc-100 font-medium"
                    : "text-zinc-400 hover:bg-zinc-800/60 hover:text-zinc-200 border border-transparent"
                }`}
              >
                {iconForKind(item.kind)}
                <span className="truncate">{item.title}</span>
              </button>
            );
          })}
        </div>
      </div>
    </aside>
  );
}
