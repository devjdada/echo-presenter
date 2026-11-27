import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Image,
  Mic,
  Sliders,
  Bot,
  ChevronDown,
  ChevronUp,
  Quote,
  BookOpen,
} from "lucide-react";
import { useShowStore, DockTab } from "../stores/useShowStore";

const isTauri = () => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export function BottomDock() {
  const {
    dockOpen,
    setDockOpen,
    dockTab,
    setDockTab,
    proVisualMode,
    triggerSlideLive,
    addGeneratedPlaylist,
  } = useShowStore();
  const [simLine, setSimLine] = useState("Let's read Romans 8:28 — all things work together for good.");
  const [detectedBooks, setDetectedBooks] = useState<string[]>(["John 8:12"]);
  const [aiInput, setAiInput] = useState("");
  const [listening, setListening] = useState(false);
  const [layerOpacity, setLayerOpacity] = useState(1.0);
  const [blendMode, setBlendMode] = useState("Normal");
  const [dockHeight, setDockHeight] = useState(240);
  const [aiMessages, setAiMessages] = useState<{ role: "user" | "ai"; text: string }[]>([
    { role: "ai", text: "Echo AI ready. Ask me to create slides, search scriptures, or paste sermon notes." },
  ]);

  const startResize = (e: React.MouseEvent) => {
    e.preventDefault();
    const startY = e.clientY;
    const startH = dockHeight;
    const onMove = (ev: MouseEvent) => {
      const maxH = Math.round(window.innerHeight * 0.65);
      const next = Math.max(140, Math.min(maxH, startH + (startY - ev.clientY)));
      setDockHeight(next);
    };
    const onUp = () => {
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    };
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
  };

  const tabs: { id: DockTab; label: string; icon: React.ReactNode; proOnly?: boolean }[] = [
    { id: "media", label: "Media Bin", icon: <Image className="w-3.5 h-3.5" /> },
    { id: "audio_intel", label: "Audio Intel (Scriptures + Quotes)", icon: <Mic className="w-3.5 h-3.5" /> },
    { id: "visual_fx", label: "Visual Clip Deck / FX", icon: <Sliders className="w-3.5 h-3.5" />, proOnly: true },
    { id: "echo_ai", label: "Echo AI Chat", icon: <Bot className="w-3.5 h-3.5" /> },
  ];

  const visibleTabs = tabs.filter((t) => !t.proOnly || proVisualMode);

  return (
    <section
      style={{ height: dockOpen ? `${dockHeight}px` : "36px" }}
      className="bg-zinc-900 border-t border-zinc-800 flex flex-col shrink-0 relative"
    >
      {dockOpen && (
        <div
          onMouseDown={startResize}
          className="h-1.5 w-full cursor-ns-resize hover:bg-indigo-500/60 transition-colors absolute top-0 left-0 z-10"
          title="Drag to resize dock"
        />
      )}
      {/* Dock Tab Bar */}
      <div className="h-9 px-3 flex items-center justify-between border-b border-zinc-800/80 bg-zinc-950/60 shrink-0">
        <div className="flex items-center gap-1">
          {visibleTabs.map((tab) => {
            const active = dockOpen && dockTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setDockTab(tab.id)}
                className={`flex items-center gap-1.5 px-2.5 py-1 rounded text-xs font-medium transition-colors ${
                  active
                    ? tab.id === "visual_fx"
                      ? "bg-violet-600/20 text-violet-300 border border-violet-500/40"
                      : "bg-zinc-800 text-white"
                    : "text-zinc-400 hover:text-zinc-200"
                }`}
              >
                {tab.icon}
                <span>{tab.label}</span>
              </button>
            );
          })}
        </div>

        <button
          onClick={() => setDockOpen(!dockOpen)}
          className="p-1 rounded hover:bg-zinc-800 text-zinc-400 hover:text-zinc-200"
          title={dockOpen ? "Collapse Dock" : "Expand Dock"}
        >
          {dockOpen ? <ChevronDown className="w-4 h-4" /> : <ChevronUp className="w-4 h-4" />}
        </button>
      </div>

      {/* Dock Content */}
      {dockOpen && (
        <div className="flex-1 overflow-y-auto p-3">
          {dockTab === "media" && (
            <div className="flex items-center gap-3">
              <button
                type="button"
                onClick={async () => {
                  if (!isTauri()) return;
                  const path = window.prompt("Enter image/video file path:");
                  if (path) {
                    await invoke("import_media_item", {
                      name: path.split(/[/\\]/).pop() ?? "Media",
                      filePath: path,
                      kind: "image",
                    });
                    await invoke("list_media_items");
                  }
                }}
                className="w-28 h-24 rounded bg-zinc-950 border border-dashed border-zinc-700 hover:border-indigo-500 flex flex-col items-center justify-center text-xs text-zinc-400"
              >
                + Import
              </button>
              {["Motion Blue Particles", "Golden Light Rays", "Abstract Worship Loop"].map(
                (item) => (
                  <div
                    key={item}
                    className="w-36 h-24 rounded bg-zinc-950 border border-zinc-800 flex flex-col items-center justify-center p-2 text-center hover:border-zinc-600 cursor-pointer"
                  >
                    <Image className="w-5 h-5 text-zinc-500 mb-1" />
                    <span className="text-[11px] text-zinc-300 truncate w-full">{item}</span>
                  </div>
                )
              )}
            </div>
          )}

          {dockTab === "audio_intel" && (
            <div className="space-y-2.5">
              <div className="flex gap-2">
                <input
                  type="text"
                  value={simLine}
                  onChange={(e) => setSimLine(e.target.value)}
                  placeholder="Simulate sermon line for live scripture & quote detection..."
                  className="flex-1 bg-zinc-950 border border-zinc-800 rounded px-2.5 py-1 text-xs text-zinc-200"
                />
                <button
                  onClick={async () => {
                    if (!isTauri()) return;
                    await invoke("audio_list_devices").catch(() => {});
                    if (listening) {
                      await invoke("audio_stop_listening");
                      setListening(false);
                    } else {
                      await invoke("audio_start_listening", { deviceName: null });
                      setListening(true);
                    }
                  }}
                  className={`px-3 py-1 rounded text-xs font-medium text-white ${
                    listening ? "bg-red-600 hover:bg-red-500" : "bg-emerald-600 hover:bg-emerald-500"
                  }`}
                >
                  {listening ? "Stop Mic" : "Start Mic"}
                </button>
                <button
                  onClick={async () => {
                    if (!isTauri()) return;
                    const res = await invoke<{
                      detected_scriptures: { book: string }[];
                    }>("audio_simulate_sermon_line", { line: simLine, speaker: "Pastor James" });
                    if (res.detected_scriptures.length > 0) {
                      setDetectedBooks(res.detected_scriptures.map((d) => d.book));
                    }
                  }}
                  className="px-3 py-1 rounded bg-indigo-600 hover:bg-indigo-500 text-xs font-medium text-white"
                >
                  Detect
                </button>
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div className="bg-zinc-950 border border-zinc-800 rounded p-2.5 flex items-start justify-between gap-2">
                  <div>
                    <div className="flex items-center gap-1.5 text-[11px] font-semibold text-amber-400">
                      <BookOpen className="w-3.5 h-3.5" />
                      Detected Scripture • {detectedBooks.join(", ")}
                    </div>
                    <p className="text-xs text-zinc-200 mt-1">{simLine}</p>
                  </div>
                  <button
                    onClick={() => triggerSlideLive("s-202")}
                    className="px-2 py-1 rounded bg-amber-600 hover:bg-amber-500 text-[11px] font-medium text-white shrink-0"
                  >
                    Go Live
                  </button>
                </div>

                <div className="bg-zinc-950 border border-zinc-800 rounded p-2.5 flex items-start justify-between gap-2">
                  <div>
                    <div className="flex items-center gap-1.5 text-[11px] font-semibold text-emerald-400">
                      <Quote className="w-3.5 h-3.5" />
                      Quotable Quote • 0.92
                    </div>
                    <p className="text-xs text-zinc-200 mt-1">
                      "God doesn't call the qualified — He qualifies the called."
                    </p>
                  </div>
                  <button
                    onClick={() => triggerSlideLive("s-203")}
                    className="px-2 py-1 rounded bg-emerald-600 hover:bg-emerald-500 text-[11px] font-medium text-white shrink-0"
                  >
                    Lower-3rd
                  </button>
                </div>
              </div>
            </div>
          )}

          {dockTab === "visual_fx" && proVisualMode && (
            <div className="flex items-center gap-4">
              {["Layer 1: Particles", "Layer 2: Glow FX", "Layer 3: Audio-Reactive"].map(
                (fx) => (
                  <div
                    key={fx}
                    className="px-3 py-2 rounded bg-violet-950/30 border border-violet-700/40 text-xs text-violet-200"
                  >
                    {fx}
                  </div>
                )
              )}
              <div className="flex items-center gap-2 text-xs text-zinc-300">
                <span>Opacity</span>
                <input
                  type="range"
                  min={0}
                  max={1}
                  step={0.05}
                  value={layerOpacity}
                  onChange={(e) => {
                    const op = Number(e.target.value);
                    setLayerOpacity(op);
                    if (isTauri()) {
                      invoke("update_visual_layer", {
                        opacity: op,
                        blendMode,
                        audioReactive: true,
                      }).catch(() => {});
                      invoke("osc_parse_command", { addr: "/layer/1/opacity", arg: op }).catch(() => {});
                    }
                  }}
                  className="w-20 accent-violet-500"
                />
                <select
                  value={blendMode}
                  onChange={(e) => {
                    setBlendMode(e.target.value);
                    if (isTauri()) {
                      invoke("update_visual_layer", {
                        opacity: layerOpacity,
                        blendMode: e.target.value,
                        audioReactive: true,
                      }).catch(() => {});
                    }
                  }}
                  className="bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-xs"
                >
                  <option value="Normal">Normal</option>
                  <option value="Add">Add</option>
                  <option value="Screen">Screen</option>
                  <option value="Overlay">Overlay</option>
                </select>
              </div>
            </div>
          )}

          {dockTab === "echo_ai" && (
            <div className="flex flex-col h-full gap-2">
              <div className="flex-1 overflow-y-auto space-y-1.5 pr-1">
                {aiMessages.map((m, idx) => (
                  <div
                    key={idx}
                    className={`px-2.5 py-1.5 rounded text-xs ${
                      m.role === "user"
                        ? "bg-indigo-600/20 text-indigo-200 ml-8"
                        : "bg-zinc-950 border border-zinc-800 text-zinc-300 mr-8"
                    }`}
                  >
                    {m.text}
                  </div>
                ))}
              </div>
              <div className="flex items-center gap-2 shrink-0">
              <input
                type="text"
                value={aiInput}
                onChange={(e) => setAiInput(e.target.value)}
                placeholder='Paste story/sermon notes or ask Echo AI (e.g. "# Faith\\n\\n> Faith is...")...'
                className="flex-1 bg-zinc-950 border border-zinc-800 rounded px-3 py-2 text-xs text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-indigo-500"
              />
              <button
                type="button"
                onClick={async () => {
                  if (!isTauri()) return;
                  const path = window.prompt("Import .pdf, .docx, .pptx, .txt, or .md path:");
                  if (path) {
                    const slides = await invoke<{ id: string; label: string; text: string }[]>(
                      "import_document_to_slides",
                      { filePath: path }
                    );
                    await addGeneratedPlaylist(
                      "Imported Document",
                      slides.map((s) => ({
                        id: s.id,
                        label: s.label,
                        group: "quote",
                        text: s.text,
                        bgGradient: "from-indigo-950 via-zinc-900 to-zinc-950",
                      }))
                    );
                  }
                }}
                className="px-3 py-2 rounded bg-zinc-800 hover:bg-zinc-700 text-xs font-medium text-zinc-200"
              >
                Import Doc
              </button>
              <button
                type="button"
                onClick={async () => {
                  if (!aiInput.trim()) return;
                  if (isTauri()) {
                    setAiMessages((prev) => [...prev, { role: "user", text: aiInput }]);
                    await invoke("ai_check_safety", { action: "create_slides" }).catch(() => {});
                    const reply = await invoke<string>("ai_chat_send", {
                      apiKey: "",
                      model: "google/gemini-2.5-flash",
                      prompt: aiInput,
                    }).catch((err) => `Note: ${String(err)} — generated slides locally.`);
                    const slides = await invoke<
                      { id: string; label: string; text: string }[]
                    >("ingest_text_to_slides", { text: aiInput });
                    await addGeneratedPlaylist(
                      "AI Generated Deck",
                      slides.map((s) => ({
                        id: s.id,
                        label: s.label,
                        group: "quote",
                        text: s.text,
                        bgGradient: "from-indigo-950 via-zinc-900 to-zinc-950",
                      }))
                    );
                    setAiMessages((prev) => [
                      ...prev,
                      { role: "ai", text: `${reply} (Created ${slides.length} slides)` },
                    ]);
                    setAiInput("");
                  }
                }}
                className="px-3 py-2 rounded bg-indigo-600 hover:bg-indigo-500 text-xs font-medium text-white"
              >
                Generate Slides
              </button>
              </div>
            </div>
          )}
        </div>
      )}
    </section>
  );
}
