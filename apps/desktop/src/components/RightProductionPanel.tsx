import { invoke } from "@tauri-apps/api/core";
import { Layers, Clock, MessageSquare, Trash2 } from "lucide-react";
import { useShowStore } from "../stores/useShowStore";

const OUTPUT_LOOKS = [
  { id: "main-fullscreen", label: "Main Full-Screen" },
  { id: "ndi-lower-third", label: "NDI Lower-Third Alpha" },
  { id: "stage-confidence", label: "Stage Confidence" },
];

const CLEAR_BUTTONS = [
  { id: "all", label: "Clear All", danger: true },
  { id: "audio", label: "Audio", danger: false },
  { id: "props", label: "Props", danger: false },
  { id: "announcements", label: "Announce", danger: false },
  { id: "slide", label: "Slide", danger: false },
  { id: "media", label: "Media", danger: false },
];

export function RightProductionPanel() {
  const {
    playlist,
    liveSlideId,
    previewMode,
    setPreviewMode,
    activeLookId,
    setOutputLook,
    clearLayer,
    clearedLayers,
    stageMessage,
    setStageMessage,
    previewPngBase64,
    gpuAdapterName,
    openOutputWindow,
  } = useShowStore();

  const allSlides = playlist.flatMap((p) => p.slides);
  const liveSlide = allSlides.find((s) => s.id === liveSlideId) ?? null;

  return (
    <aside className="w-80 bg-zinc-900/90 border-l border-zinc-800 flex flex-col shrink-0 overflow-y-auto">
      {/* Live Output Preview Header */}
      <div className="h-8 px-3 border-b border-zinc-800 flex items-center justify-between bg-zinc-950/40">
        <span className="text-[11px] font-semibold uppercase tracking-wider text-zinc-400">
          Live Output
        </span>
        <div className="flex items-center bg-zinc-950 rounded p-0.5 border border-zinc-800">
          <button
            onClick={() => setPreviewMode("audience")}
            className={`px-2 py-0.5 text-[10px] font-medium rounded ${
              previewMode === "audience"
                ? "bg-zinc-800 text-white"
                : "text-zinc-500 hover:text-zinc-300"
            }`}
          >
            Audience
          </button>
          <button
            onClick={() => setPreviewMode("stage")}
            className={`px-2 py-0.5 text-[10px] font-medium rounded ${
              previewMode === "stage"
                ? "bg-zinc-800 text-white"
                : "text-zinc-500 hover:text-zinc-300"
            }`}
          >
            Stage
          </button>
        </div>
      </div>

      {/* 16:9 Live Output Monitor */}
      <div className="p-3 border-b border-zinc-800 space-y-2.5">
        <div
          className={`aspect-video w-full rounded border border-zinc-700/80 overflow-hidden relative flex items-center justify-center p-4 text-center ${
            activeLookId === "ndi-lower-third"
              ? "bg-[radial-gradient(#27272a_1px,transparent_1px)] [background-size:10px_10px] bg-zinc-950"
              : liveSlide
              ? `bg-gradient-to-br ${liveSlide.bgGradient}`
              : "bg-black"
          }`}
        >
          {previewPngBase64 ? (
            <img
              src={`data:image/png;base64,${previewPngBase64}`}
              alt="GPU Live Preview"
              className="w-full h-full object-contain"
            />
          ) : liveSlide ? (
            activeLookId === "ndi-lower-third" ? (
              <div className="absolute bottom-3 left-3 right-3 bg-zinc-950/90 border-l-4 border-amber-500 px-3 py-2 rounded-r text-left shadow-lg">
                <p className="text-[11px] font-semibold text-white line-clamp-2">
                  {liveSlide.text}
                </p>
                {liveSlide.reference && (
                  <p className="text-[9px] text-amber-400 mt-0.5">{liveSlide.reference}</p>
                )}
              </div>
            ) : (
              <div>
                <p className="text-xs font-semibold text-white whitespace-pre-line">
                  {liveSlide.text}
                </p>
                {liveSlide.reference && (
                  <p className="mt-1.5 text-[10px] font-medium text-amber-300">
                    {liveSlide.reference}
                  </p>
                )}
              </div>
            )
          ) : (
            <span className="text-[11px] text-zinc-600 font-mono uppercase">
              Output Clear
            </span>
          )}

          <span className="absolute top-1.5 left-1.5 px-1.5 py-0.5 rounded bg-black/70 text-[9px] font-mono text-zinc-300 border border-zinc-700/60">
            {previewMode === "audience" ? "AUDIENCE" : "STAGE"}
          </span>
        </div>

        {/* Output Look Selector */}
        <div className="flex items-center justify-between gap-2">
          <span className="flex items-center gap-1 text-[11px] text-zinc-400 shrink-0">
            <Layers className="w-3.5 h-3.5 text-indigo-400" />
            Look:
          </span>
          <select
            value={activeLookId}
            onChange={(e) => setOutputLook(e.target.value)}
            className="flex-1 bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-xs text-zinc-200 focus:outline-none focus:border-indigo-500"
          >
            {OUTPUT_LOOKS.map((look) => (
              <option key={look.id} value={look.id}>
                {look.label}
              </option>
            ))}
          </select>
        </div>

        {/* Output Window Launcher & GPU Badge */}
        <div className="space-y-1.5 pt-1">
          <div className="flex items-center justify-between gap-2">
            <button
              type="button"
              onClick={() => {
                invoke("list_monitors").catch(() => {});
                openOutputWindow("windowed");
              }}
              className="px-2.5 py-1 rounded bg-indigo-600/20 border border-indigo-500/40 hover:bg-indigo-600/30 text-[11px] font-medium text-indigo-300 transition-colors whitespace-nowrap"
            >
              Open GPU Output (1280×720)
            </button>
            <select
              onChange={(e) => {
                invoke("set_transition", { kind: e.target.value, durationMs: 400 }).catch(() => {});
              }}
              className="bg-zinc-950 border border-zinc-800 rounded px-1.5 py-1 text-[11px] text-zinc-300"
              title="Transition"
            >
              <option value="cut">Cut</option>
              <option value="fade">Fade (400ms)</option>
              <option value="dissolve">Dissolve (400ms)</option>
            </select>
          </div>
          <div className="flex items-center justify-between text-[10px] text-zinc-500">
            <span>NDI: EchoPresenter - Lower Third Alpha</span>
            <span className="text-zinc-400">Standby</span>
          </div>
          {gpuAdapterName && (
            <div className="text-[10px] text-zinc-500 truncate" title={gpuAdapterName}>
              GPU: {gpuAdapterName}
            </div>
          )}
        </div>
      </div>

      {/* One-Click Layer Clear Bar */}
      <div className="p-3 border-b border-zinc-800">
        <div className="text-[10px] font-semibold uppercase tracking-wider text-zinc-500 mb-2 flex items-center gap-1">
          <Trash2 className="w-3 h-3" />
          Clear Layers
        </div>
        <div className="grid grid-cols-3 gap-1.5">
          {CLEAR_BUTTONS.map((btn) => {
            const isCleared = clearedLayers.includes(btn.id);
            return (
              <button
                key={btn.id}
                onClick={() => clearLayer(btn.id)}
                className={`px-2 py-1.5 rounded text-[11px] font-medium border transition-colors ${
                  btn.danger
                    ? "bg-red-950/50 border-red-800/70 text-red-300 hover:bg-red-900/60"
                    : isCleared
                    ? "bg-zinc-950 border-zinc-800 text-zinc-600"
                    : "bg-zinc-950 border-zinc-800 text-zinc-300 hover:bg-zinc-800 hover:text-white"
                }`}
              >
                {btn.label}
              </button>
            );
          })}
        </div>
      </div>

      {/* Countdown Timer & Stage Message */}
      <div className="p-3 space-y-3 flex-1 overflow-y-auto">
        <div className="bg-zinc-950 border border-zinc-800 rounded p-2.5 flex items-center justify-between">
          <div className="flex items-center gap-1.5 text-xs text-zinc-400">
            <Clock className="w-3.5 h-3.5 text-emerald-400" />
            <span>Service Timer</span>
          </div>
          <span className="font-mono text-sm font-semibold text-emerald-400">05:00</span>
        </div>

        <div className="space-y-1.5">
          <label className="flex items-center gap-1.5 text-[11px] text-zinc-400">
            <MessageSquare className="w-3.5 h-3.5 text-amber-400" />
            Stage Message
          </label>
          <div className="flex gap-1.5">
            <input
              type="text"
              value={stageMessage}
              onChange={(e) => setStageMessage(e.target.value)}
              placeholder="Send note to stage..."
              className="flex-1 bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-xs text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-indigo-500"
            />
            <button
              type="button"
              className="px-2.5 py-1 rounded bg-zinc-800 hover:bg-zinc-700 text-xs font-medium text-zinc-200"
            >
              Send
            </button>
          </div>
        </div>
      </div>
    </aside>
  );
}
