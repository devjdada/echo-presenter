import { useState } from "react";
import {
  Undo2,
  Redo2,
  Type,
  Image,
  Square,
  Radio,
  Plus,
  Copy,
  Trash2,
  Bold,
  Italic,
  AlignLeft,
  AlignCenter,
  AlignRight,
} from "lucide-react";
import { useShowStore, SlideGroupKind, SlideItem } from "../stores/useShowStore";

const BG_PRESETS = [
  { label: "Deep Indigo", value: "from-slate-900 via-blue-950 to-slate-950" },
  { label: "Teal Worship", value: "from-teal-950 via-emerald-950 to-zinc-950" },
  { label: "Amber Scripture", value: "from-amber-950 via-zinc-900 to-zinc-950" },
  { label: "Purple Bridge", value: "from-purple-950 via-indigo-950 to-zinc-950" },
];

export function SlideEditor() {
  const {
    playlist,
    activePlaylistId,
    previewSlideId,
    selectPreviewSlide,
    triggerSlideLive,
    addGeneratedPlaylist,
  } = useShowStore();
  const currentItem = playlist.find((p) => p.id === activePlaylistId) ?? playlist[0];
  const slides = currentItem?.slides ?? [];
  const activeSlide = slides.find((s) => s.id === previewSlideId) ?? slides[0];

  const [text, setText] = useState(activeSlide?.text ?? "Edit slide text...");
  const [reference, setReference] = useState(activeSlide?.reference ?? "");
  const [fontSize, setFontSize] = useState(64);
  const [fontFamily, setFontFamily] = useState("Segoe UI");
  const [bold, setBold] = useState(true);
  const [italic, setItalic] = useState(false);
  const [align, setAlign] = useState<"left" | "center" | "right">("center");
  const [textColor, setTextColor] = useState("#FFFFFF");
  const [dropShadow, setDropShadow] = useState(true);
  const [group, setGroup] = useState<SlideGroupKind>(activeSlide?.group ?? "verse");
  const [bgGradient, setBgGradient] = useState(
    activeSlide?.bgGradient ?? BG_PRESETS[0].value
  );
  const [undoStack, setUndoStack] = useState<string[]>([]);
  const [redoStack, setRedoStack] = useState<string[]>([]);

  const handleTextChange = (next: string) => {
    setUndoStack((s) => [...s.slice(-19), text]);
    setRedoStack([]);
    setText(next);
  };

  const handleUndo = () => {
    if (undoStack.length === 0) return;
    const prev = undoStack[undoStack.length - 1];
    setRedoStack((r) => [...r, text]);
    setUndoStack((s) => s.slice(0, -1));
    setText(prev);
  };

  const handleRedo = () => {
    if (redoStack.length === 0) return;
    const next = redoStack[redoStack.length - 1];
    setUndoStack((s) => [...s, text]);
    setRedoStack((r) => r.slice(0, -1));
    setText(next);
  };

  const handleAddSlide = () => {
    const newSlide: SlideItem = {
      id: `s-${Date.now()}`,
      label: `Slide ${slides.length + 1}`,
      group: "verse",
      text: "New Slide Text",
      bgGradient,
    };
    addGeneratedPlaylist(currentItem?.title ?? "Deck", [...slides, newSlide]);
    selectPreviewSlide(newSlide.id);
    setText(newSlide.text);
  };

  return (
    <div className="flex-1 min-h-0 flex bg-zinc-950 text-zinc-100">
      {/* Left Toolbar */}
      <aside className="w-12 bg-zinc-900 border-r border-zinc-800 flex flex-col items-center py-3 gap-2.5 shrink-0">
        <button
          onClick={() => handleTextChange(`${text}\nNew Text`)}
          className="p-2 rounded bg-zinc-800 text-zinc-100"
          title="Add Text"
        >
          <Type className="w-4 h-4" />
        </button>
        <button
          onClick={() => setBgGradient(BG_PRESETS[1].value)}
          className="p-2 rounded hover:bg-zinc-800 text-zinc-400"
          title="Cycle Background"
        >
          <Image className="w-4 h-4" />
        </button>
        <button
          onClick={() => setDropShadow((v) => !v)}
          className="p-2 rounded hover:bg-zinc-800 text-zinc-400"
          title="Toggle Accent / Shadow"
        >
          <Square className="w-4 h-4" />
        </button>
        <div className="border-t border-zinc-800 w-7 my-1" />
        <button onClick={handleUndo} className="p-2 rounded hover:bg-zinc-800 text-zinc-400" title="Undo">
          <Undo2 className="w-4 h-4" />
        </button>
        <button onClick={handleRedo} className="p-2 rounded hover:bg-zinc-800 text-zinc-400" title="Redo">
          <Redo2 className="w-4 h-4" />
        </button>
      </aside>

      {/* Left Slide Filmstrip */}
      <aside className="w-48 bg-zinc-900/70 border-r border-zinc-800 flex flex-col shrink-0">
        <div className="h-9 px-2.5 border-b border-zinc-800 flex items-center justify-between">
          <span className="text-[11px] font-semibold uppercase tracking-wider text-zinc-400">
            Slides ({slides.length})
          </span>
          <div className="flex items-center gap-1">
            <button onClick={handleAddSlide} className="p-1 rounded hover:bg-zinc-800 text-zinc-300" title="Add Slide">
              <Plus className="w-3.5 h-3.5" />
            </button>
            <button onClick={handleAddSlide} className="p-1 rounded hover:bg-zinc-800 text-zinc-400" title="Duplicate">
              <Copy className="w-3.5 h-3.5" />
            </button>
            <button className="p-1 rounded hover:bg-zinc-800 text-zinc-500" title="Delete">
              <Trash2 className="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
        <div className="flex-1 overflow-y-auto p-2 space-y-2">
          {slides.map((s, idx) => (
            <button
              key={s.id}
              onClick={() => {
                selectPreviewSlide(s.id);
                setText(s.text);
                setReference(s.reference ?? "");
                setGroup(s.group);
                setBgGradient(s.bgGradient);
              }}
              className={`w-full text-left rounded border overflow-hidden ${
                s.id === activeSlide?.id ? "border-indigo-500" : "border-zinc-800 hover:border-zinc-600"
              }`}
            >
              <div className={`aspect-video bg-gradient-to-br ${s.bgGradient} p-2 flex items-center justify-center text-center`}>
                <span className="text-[10px] text-white line-clamp-3">{s.text}</span>
              </div>
              <div className="px-2 py-0.5 bg-zinc-900 text-[10px] text-zinc-400 flex justify-between">
                <span>{s.label}</span>
                <span>{idx + 1}</span>
              </div>
            </button>
          ))}
        </div>
      </aside>

      {/* Center: 16:9 WYSIWYG Canvas */}
      <main className="flex-1 flex items-center justify-center p-6 bg-zinc-950 min-w-0">
        <div
          className={`aspect-video w-full max-w-3xl rounded-lg border border-zinc-700 shadow-2xl bg-gradient-to-br ${bgGradient} flex flex-col items-center justify-center p-10`}
        >
          <textarea
            value={text}
            onChange={(e) => handleTextChange(e.target.value)}
            style={{
              fontSize: `${Math.round(fontSize * 0.45)}px`,
              fontFamily,
              fontWeight: bold ? 700 : 400,
              fontStyle: italic ? "italic" : "normal",
              textAlign: align,
              color: textColor,
              textShadow: dropShadow ? "2px 2px 8px rgba(0,0,0,0.85)" : "none",
            }}
            className="w-full bg-transparent resize-none focus:outline-none leading-tight"
            rows={4}
          />
          {reference && (
            <div className="mt-3 text-sm font-semibold text-amber-300 uppercase tracking-wider">
              {reference}
            </div>
          )}
        </div>
      </main>

      {/* Right: Inspector */}
      <aside className="w-72 bg-zinc-900 border-l border-zinc-800 p-4 space-y-3 overflow-y-auto shrink-0">
        <div className="text-xs font-semibold uppercase tracking-wider text-zinc-400">
          Slide Inspector
        </div>

        <div className="space-y-1">
          <label className="text-[11px] text-zinc-400">Group</label>
          <select
            value={group}
            onChange={(e) => setGroup(e.target.value as SlideGroupKind)}
            className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1.5 text-xs text-zinc-200"
          >
            <option value="verse">Verse (Blue)</option>
            <option value="chorus">Chorus (Teal)</option>
            <option value="bridge">Bridge (Purple)</option>
            <option value="scripture">Scripture (Amber)</option>
            <option value="quote">Quote (Emerald)</option>
            <option value="announcement">Announcement (Rose)</option>
          </select>
        </div>

        <div className="space-y-1">
          <label className="text-[11px] text-zinc-400">Font Family</label>
          <select
            value={fontFamily}
            onChange={(e) => setFontFamily(e.target.value)}
            className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1.5 text-xs text-zinc-200"
          >
            <option value="Segoe UI">Segoe UI</option>
            <option value="Inter">Inter</option>
            <option value="Georgia">Georgia</option>
            <option value="Consolas">Consolas</option>
          </select>
        </div>

        <div className="flex items-center justify-between gap-1">
          <div className="flex gap-1">
            <button
              onClick={() => setBold((v) => !v)}
              className={`p-1.5 rounded border ${bold ? "bg-indigo-600/30 border-indigo-500 text-white" : "bg-zinc-950 border-zinc-800 text-zinc-400"}`}
            >
              <Bold className="w-3.5 h-3.5" />
            </button>
            <button
              onClick={() => setItalic((v) => !v)}
              className={`p-1.5 rounded border ${italic ? "bg-indigo-600/30 border-indigo-500 text-white" : "bg-zinc-950 border-zinc-800 text-zinc-400"}`}
            >
              <Italic className="w-3.5 h-3.5" />
            </button>
          </div>
          <div className="flex gap-1">
            <button onClick={() => setAlign("left")} className={`p-1.5 rounded border ${align === "left" ? "bg-zinc-800 border-zinc-600" : "bg-zinc-950 border-zinc-800 text-zinc-500"}`}>
              <AlignLeft className="w-3.5 h-3.5" />
            </button>
            <button onClick={() => setAlign("center")} className={`p-1.5 rounded border ${align === "center" ? "bg-zinc-800 border-zinc-600" : "bg-zinc-950 border-zinc-800 text-zinc-500"}`}>
              <AlignCenter className="w-3.5 h-3.5" />
            </button>
            <button onClick={() => setAlign("right")} className={`p-1.5 rounded border ${align === "right" ? "bg-zinc-800 border-zinc-600" : "bg-zinc-950 border-zinc-800 text-zinc-500"}`}>
              <AlignRight className="w-3.5 h-3.5" />
            </button>
          </div>
          <input
            type="color"
            value={textColor}
            onChange={(e) => setTextColor(e.target.value)}
            className="w-7 h-7 rounded bg-transparent border border-zinc-800 cursor-pointer"
          />
        </div>

        <div className="space-y-1">
          <label className="text-[11px] text-zinc-400">Font Size ({fontSize}px)</label>
          <input
            type="range"
            min={28}
            max={96}
            value={fontSize}
            onChange={(e) => setFontSize(Number(e.target.value))}
            className="w-full accent-indigo-500"
          />
        </div>

        <div className="space-y-1">
          <label className="text-[11px] text-zinc-400">Background Preset</label>
          <select
            value={bgGradient}
            onChange={(e) => setBgGradient(e.target.value)}
            className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1.5 text-xs text-zinc-200"
          >
            {BG_PRESETS.map((p) => (
              <option key={p.value} value={p.value}>
                {p.label}
              </option>
            ))}
          </select>
        </div>

        <div className="space-y-1">
          <label className="text-[11px] text-zinc-400">Reference / Attribution</label>
          <input
            type="text"
            value={reference}
            onChange={(e) => setReference(e.target.value)}
            placeholder="e.g. John 3:16 (KJV)"
            className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1.5 text-xs text-zinc-200"
          />
        </div>

        {activeSlide && (
          <button
            onClick={() => triggerSlideLive(activeSlide.id)}
            className="w-full flex items-center justify-center gap-1.5 py-2 rounded bg-red-600 hover:bg-red-500 text-xs font-semibold text-white"
          >
            <Radio className="w-3.5 h-3.5" />
            Send Live
          </button>
        )}
      </aside>
    </div>
  );
}
