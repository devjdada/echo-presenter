import { useState } from "react";
import { Search, Settings, Sparkles, Monitor, Radio } from "lucide-react";
import { useShowStore, WorkspaceTab } from "../stores/useShowStore";
import { SettingsModal } from "./SettingsModal";
import logoHorizontal from "../assets/logo-horizontal.svg";

const TABS: WorkspaceTab[] = ["Show", "Edit", "Bible", "Reflow"];

export function TopBar() {
  const { workspaceTab, setWorkspaceTab, proVisualMode, toggleProVisualMode, audienceLive } =
    useShowStore();
  const [settingsOpen, setSettingsOpen] = useState(false);

  return (
    <header className="h-11 bg-zinc-900 border-b border-zinc-800 px-3 flex items-center justify-between shrink-0">
      {/* Left: Brand & Workspace Tabs */}
      <div className="flex items-center gap-4">
        <img src={logoHorizontal} className="h-7 w-auto select-none" alt="EchoPresenter" />

        <nav className="flex items-center bg-zinc-950/70 p-0.5 rounded-md border border-zinc-800/80">
          {TABS.map((tab) => (
            <button
              key={tab}
              onClick={() => setWorkspaceTab(tab)}
              className={`px-3 py-1 text-xs font-medium rounded transition-colors ${
                workspaceTab === tab
                  ? "bg-zinc-800 text-white shadow-sm"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              {tab}
            </button>
          ))}
        </nav>
      </div>

      {/* Center: Quick Search */}
      <button
        type="button"
        className="flex items-center gap-2 px-3 py-1 rounded bg-zinc-950 border border-zinc-800 text-xs text-zinc-400 hover:text-zinc-200 hover:border-zinc-700 transition-colors w-72 justify-between whitespace-nowrap"
      >
        <span className="flex items-center gap-1.5 truncate">
          <Search className="w-3.5 h-3.5 shrink-0" />
          <span className="truncate">Quick Search (Bible, Songs, Slides)...</span>
        </span>
        <kbd className="text-[10px] px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-400 font-mono shrink-0">
          Ctrl+K
        </kbd>
      </button>

      {/* Right: Mode Toggle, Output Status, Settings */}
      <div className="flex items-center gap-2.5">
        <button
          onClick={toggleProVisualMode}
          className={`flex items-center gap-1.5 px-2.5 py-1 rounded text-xs font-medium border transition-colors ${
            proVisualMode
              ? "bg-violet-950/60 border-violet-600/60 text-violet-200"
              : "bg-zinc-950 border-zinc-800 text-zinc-400 hover:text-zinc-200"
          }`}
          title="Toggle between Volunteer Mode and Pro Visual Mode"
        >
          <Sparkles className="w-3.5 h-3.5" />
          {proVisualMode ? "Pro Visual Mode" : "Volunteer Mode"}
        </button>

        <div
          className={`flex items-center gap-1.5 px-2.5 py-1 rounded text-xs font-semibold border ${
            audienceLive
              ? "bg-red-950/60 border-red-600/60 text-red-300"
              : "bg-zinc-950 border-zinc-800 text-zinc-500"
          }`}
        >
          {audienceLive ? (
            <Radio className="w-3.5 h-3.5 text-red-400 animate-pulse" />
          ) : (
            <Monitor className="w-3.5 h-3.5" />
          )}
          <span>Audience: {audienceLive ? "LIVE" : "OFF"}</span>
        </div>

        <button
          type="button"
          onClick={() => setSettingsOpen(true)}
          className="p-1.5 rounded hover:bg-zinc-800 text-zinc-400 hover:text-zinc-100 transition-colors"
          title="Settings"
        >
          <Settings className="w-4 h-4" />
        </button>
      </div>
      <SettingsModal open={settingsOpen} onClose={() => setSettingsOpen(false)} />
    </header>
  );
}
