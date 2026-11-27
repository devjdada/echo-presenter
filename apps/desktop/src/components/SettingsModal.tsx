import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Shield, Key, Globe, Eye, EyeOff, Copy, CheckCircle2 } from "lucide-react";
import logoFull from "../assets/logo-full.svg";

const isTauri = () => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export function SettingsModal({ open, onClose }: { open: boolean; onClose: () => void }) {
  const [openRouterKey, setOpenRouterKey] = useState("");
  const [showOpenRouter, setShowOpenRouter] = useState(false);
  const [chatModel, setChatModel] = useState("google/gemini-2.5-flash");
  const [quoteModel, setQuoteModel] = useState("google/gemini-2.5-flash");
  const [sttProvider, setSttProvider] = useState("groq-whisper");
  const [sttKey, setSttKey] = useState("");
  const [ollamaUrl, setOllamaUrl] = useState("http://localhost:11434");
  const [ollamaModel, setOllamaModel] = useState("llama3.1:8b");
  const [restPort, setRestPort] = useState("50001");
  const [oscPort, setOscPort] = useState("7000");
  const [mcpApiKey, setMcpApiKey] = useState("ep-local-dev-key");
  const [requireApproval, setRequireApproval] = useState(true);
  const [mcpToolCount, setMcpToolCount] = useState(14);
  const [connStatus, setConnStatus] = useState<string | null>(null);
  const [whisperModels, setWhisperModels] = useState<Array<{ id: string; size_label: string; installed: boolean; path: string }>>([]);
  const [downloadingModel, setDownloadingModel] = useState<string | null>(null);

  useEffect(() => {
    if (!open || !isTauri()) return;
    invoke<string | null>("load_settings")
      .then((raw) => {
        if (raw) {
          const p = JSON.parse(raw);
          if (p.openRouterKey) setOpenRouterKey(p.openRouterKey);
          if (p.chatModel) setChatModel(p.chatModel);
          if (p.quoteModel) setQuoteModel(p.quoteModel);
          if (p.sttProvider) setSttProvider(p.sttProvider);
          if (p.sttKey) setSttKey(p.sttKey);
          if (p.ollamaUrl) setOllamaUrl(p.ollamaUrl);
          if (p.ollamaModel) setOllamaModel(p.ollamaModel);
          if (p.restPort) setRestPort(p.restPort);
          if (p.oscPort) setOscPort(p.oscPort);
          if (p.mcpApiKey) setMcpApiKey(p.mcpApiKey);
          if (typeof p.requireApproval === "boolean") setRequireApproval(p.requireApproval);
        }
      })
      .catch(console.error);
    invoke<unknown[]>("mcp_list_tools")
      .then((tools) => setMcpToolCount(tools.length))
      .catch(console.error);
  }, [open]);

  if (!open) return null;

  const handleSave = async () => {
    if (isTauri()) {
      await invoke("save_settings", {
        settingsJson: JSON.stringify({
          openRouterKey,
          chatModel,
          quoteModel,
          sttProvider,
          sttKey,
          ollamaUrl,
          ollamaModel,
          restPort,
          oscPort,
          mcpApiKey,
          requireApproval,
        }),
      }).catch(console.error);
    }
    onClose();
  };

  const tunnelCmd = `cloudflared tunnel --url http://127.0.0.1:${restPort}/mcp`;

  return (
    <div className="fixed inset-0 z-50 bg-black/75 flex items-center justify-center p-4">
      <div className="w-full max-w-2xl bg-zinc-900 border border-zinc-700 rounded-lg shadow-2xl overflow-hidden">
        <div className="px-4 py-3 border-b border-zinc-800 flex items-center justify-between">
          <span className="text-sm font-semibold text-zinc-100">EchoPresenter Settings</span>
          <button onClick={onClose} className="text-zinc-400 hover:text-zinc-100">
            <X className="w-4 h-4" />
          </button>
        </div>

        <div className="p-4 space-y-4 max-h-[75vh] overflow-y-auto text-xs">
          {/* About EchoPresenter Banner */}
          <div className="p-3 rounded-lg bg-zinc-950 border border-zinc-800 flex items-center gap-4">
            <img src={logoFull} className="h-16 w-auto select-none shrink-0" alt="EchoPresenter" />
            <div className="space-y-1">
              <div className="text-xs font-semibold text-zinc-100">
                v0.1.0-beta.5 (Sprint 5)
              </div>
              <div className="text-[11px] text-zinc-400">
                Architectural Presentation &amp; Intelligent Live Production Platform
              </div>
              <div className="flex items-center gap-3 pt-1 text-[10px] text-zinc-500">
                <span className="text-emerald-400">● SQLite DB Ready</span>
                <span className="text-emerald-400">● MCP Bridge ({mcpToolCount} tools active)</span>
              </div>
            </div>
          </div>
          <div className="space-y-1.5">
            <label className="flex items-center gap-1.5 font-semibold text-zinc-300">
              <Key className="w-3.5 h-3.5 text-indigo-400" />
              OpenRouter API Key
            </label>
            <div className="flex gap-2">
              <input
                type={showOpenRouter ? "text" : "password"}
                value={openRouterKey}
                onChange={(e) => setOpenRouterKey(e.target.value)}
                placeholder="sk-or-v1-..."
                className="flex-1 bg-zinc-950 border border-zinc-800 rounded px-3 py-2 text-zinc-200"
              />
              <button
                type="button"
                onClick={() => setShowOpenRouter((v) => !v)}
                className="px-2.5 rounded bg-zinc-800 text-zinc-300"
              >
                {showOpenRouter ? <EyeOff className="w-3.5 h-3.5" /> : <Eye className="w-3.5 h-3.5" />}
              </button>
              <button
                type="button"
                onClick={async () => {
                  if (!isTauri()) return;
                  const res = await invoke<string>("test_ai_connection", {
                    apiKey: openRouterKey,
                    ollamaUrl,
                  }).catch((e) => `Error: ${String(e)}`);
                  setConnStatus(res);
                }}
                className="px-3 rounded bg-indigo-600/20 border border-indigo-500/40 text-indigo-200 font-medium"
              >
                Test
              </button>
            </div>
            {connStatus && (
              <div className="flex items-center gap-1 text-[11px] text-emerald-400">
                <CheckCircle2 className="w-3 h-3" /> {connStatus}
              </div>
            )}
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div className="space-y-1">
              <label className="font-semibold text-zinc-300">OpenRouter Chat Model</label>
              <input
                type="text"
                value={chatModel}
                onChange={(e) => setChatModel(e.target.value)}
                className="w-full bg-zinc-950 border border-zinc-800 rounded px-2.5 py-1.5 text-zinc-200"
              />
            </div>
            <div className="space-y-1">
              <label className="font-semibold text-zinc-300">Fast Quote Model</label>
              <input
                type="text"
                value={quoteModel}
                onChange={(e) => setQuoteModel(e.target.value)}
                className="w-full bg-zinc-950 border border-zinc-800 rounded px-2.5 py-1.5 text-zinc-200"
              />
            </div>
          </div>

          {sttProvider === "local-whisper" && (
            <div className="rounded-lg border border-zinc-800 bg-zinc-950/70 p-3 space-y-2">
              <div className="flex items-center justify-between">
                <span className="font-semibold text-zinc-200">Offline Whisper Models (AppData/ai-models)</span>
                <button
                  type="button"
                  onClick={() => {
                    if (isTauri()) {
                      invoke<Array<{ id: string; size_label: string; installed: boolean; path: string }>>("list_whisper_models")
                        .then(setWhisperModels)
                        .catch(() => {});
                    }
                  }}
                  className="text-[11px] text-indigo-400 hover:text-indigo-300"
                >
                  Refresh
                </button>
              </div>
              <div className="space-y-1.5">
                {(whisperModels.length > 0
                  ? whisperModels
                  : [
                      { id: "ggml-base.en.bin", size_label: "142 MB", installed: false, path: "" },
                      { id: "ggml-small.en.bin", size_label: "466 MB", installed: false, path: "" },
                    ]
                ).map((m) => (
                  <div key={m.id} className="flex items-center justify-between text-[11px] bg-zinc-900/70 rounded px-2.5 py-1.5">
                    <div>
                      <span className="font-mono text-zinc-200">{m.id}</span>
                      <span className="ml-2 text-zinc-500">({m.size_label})</span>
                    </div>
                    {m.installed ? (
                      <span className="text-emerald-400 font-medium">Installed</span>
                    ) : (
                      <button
                        type="button"
                        disabled={downloadingModel === m.id}
                        onClick={async () => {
                          if (!isTauri()) return;
                          setDownloadingModel(m.id);
                          try {
                            await invoke("download_whisper_model", { modelId: m.id });
                            const updated = await invoke<Array<{ id: string; size_label: string; installed: boolean; path: string }>>("list_whisper_models");
                            setWhisperModels(updated);
                          } catch (e) {
                            setConnStatus(`Whisper download error: ${String(e)}`);
                          } finally {
                            setDownloadingModel(null);
                          }
                        }}
                        className="px-2 py-0.5 rounded bg-amber-600/20 border border-amber-500/40 text-amber-200 hover:bg-amber-600/30 disabled:opacity-50"
                      >
                        {downloadingModel === m.id ? "Downloading..." : "Download"}
                      </button>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          <div className="grid grid-cols-2 gap-3">
            <div className="space-y-1">
              <label className="font-semibold text-zinc-300">Cloud STT Provider</label>
              <select
                value={sttProvider}
                onChange={(e) => setSttProvider(e.target.value)}
                className="w-full bg-zinc-950 border border-zinc-800 rounded px-2.5 py-1.5 text-zinc-200"
              >
                <option value="groq-whisper">Groq Whisper (Fast)</option>
                <option value="deepgram">Deepgram Nova-2</option>
                <option value="local-whisper">Local Whisper (Offline)</option>
                <option value="simulated">Simulated Test Mode</option>
              </select>
            </div>
            <div className="space-y-1">
              <label className="font-semibold text-zinc-300">STT API Key</label>
              <input
                type="password"
                value={sttKey}
                onChange={(e) => setSttKey(e.target.value)}
                placeholder="gsk_... / dg_..."
                className="w-full bg-zinc-950 border border-zinc-800 rounded px-2.5 py-1.5 text-zinc-200"
              />
            </div>
          </div>

          {sttProvider === "local-whisper" && (
            <div className="rounded-lg border border-zinc-800 bg-zinc-950/70 p-3 space-y-2">
              <div className="flex items-center justify-between">
                <span className="font-semibold text-zinc-200">Offline Whisper Models (AppData/ai-models)</span>
                <button
                  type="button"
                  onClick={() => {
                    if (isTauri()) {
                      invoke<Array<{ id: string; size_label: string; installed: boolean; path: string }>>("list_whisper_models")
                        .then(setWhisperModels)
                        .catch(() => {});
                    }
                  }}
                  className="text-[11px] text-indigo-400 hover:text-indigo-300"
                >
                  Refresh
                </button>
              </div>
              <div className="space-y-1.5">
                {(whisperModels.length > 0
                  ? whisperModels
                  : [
                      { id: "ggml-base.en.bin", size_label: "142 MB", installed: false, path: "" },
                      { id: "ggml-small.en.bin", size_label: "466 MB", installed: false, path: "" },
                    ]
                ).map((m) => (
                  <div key={m.id} className="flex items-center justify-between text-[11px] bg-zinc-900/70 rounded px-2.5 py-1.5">
                    <div>
                      <span className="font-mono text-zinc-200">{m.id}</span>
                      <span className="ml-2 text-zinc-500">({m.size_label})</span>
                    </div>
                    {m.installed ? (
                      <span className="text-emerald-400 font-medium">Installed</span>
                    ) : (
                      <button
                        type="button"
                        disabled={downloadingModel === m.id}
                        onClick={async () => {
                          if (!isTauri()) return;
                          setDownloadingModel(m.id);
                          try {
                            await invoke("download_whisper_model", { modelId: m.id });
                            const updated = await invoke<Array<{ id: string; size_label: string; installed: boolean; path: string }>>("list_whisper_models");
                            setWhisperModels(updated);
                          } catch (e) {
                            setConnStatus(`Whisper download error: ${String(e)}`);
                          } finally {
                            setDownloadingModel(null);
                          }
                        }}
                        className="px-2 py-0.5 rounded bg-amber-600/20 border border-amber-500/40 text-amber-200 hover:bg-amber-600/30 disabled:opacity-50"
                      >
                        {downloadingModel === m.id ? "Downloading..." : "Download"}
                      </button>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          <div className="grid grid-cols-3 gap-3">
            <div className="space-y-1">
              <label className="font-semibold text-zinc-300">Ollama Model</label>
              <input
                type="text"
                value={ollamaModel}
                onChange={(e) => setOllamaModel(e.target.value)}
                className="w-full bg-zinc-950 border border-zinc-800 rounded px-2.5 py-1.5 text-zinc-200"
              />
            </div>
            <div className="space-y-1">
              <label className="font-semibold text-zinc-300">REST/MCP Port</label>
              <input
                type="text"
                value={restPort}
                onChange={(e) => setRestPort(e.target.value)}
                className="w-full bg-zinc-950 border border-zinc-800 rounded px-2.5 py-1.5 text-zinc-200"
              />
            </div>
            <div className="space-y-1">
              <label className="font-semibold text-zinc-300">OSC UDP Port</label>
              <input
                type="text"
                value={oscPort}
                onChange={(e) => setOscPort(e.target.value)}
                className="w-full bg-zinc-950 border border-zinc-800 rounded px-2.5 py-1.5 text-zinc-200"
              />
            </div>
          </div>

          {sttProvider === "local-whisper" && (
            <div className="rounded-lg border border-zinc-800 bg-zinc-950/70 p-3 space-y-2">
              <div className="flex items-center justify-between">
                <span className="font-semibold text-zinc-200">Offline Whisper Models (AppData/ai-models)</span>
                <button
                  type="button"
                  onClick={() => {
                    if (isTauri()) {
                      invoke<Array<{ id: string; size_label: string; installed: boolean; path: string }>>("list_whisper_models")
                        .then(setWhisperModels)
                        .catch(() => {});
                    }
                  }}
                  className="text-[11px] text-indigo-400 hover:text-indigo-300"
                >
                  Refresh
                </button>
              </div>
              <div className="space-y-1.5">
                {(whisperModels.length > 0
                  ? whisperModels
                  : [
                      { id: "ggml-base.en.bin", size_label: "142 MB", installed: false, path: "" },
                      { id: "ggml-small.en.bin", size_label: "466 MB", installed: false, path: "" },
                    ]
                ).map((m) => (
                  <div key={m.id} className="flex items-center justify-between text-[11px] bg-zinc-900/70 rounded px-2.5 py-1.5">
                    <div>
                      <span className="font-mono text-zinc-200">{m.id}</span>
                      <span className="ml-2 text-zinc-500">({m.size_label})</span>
                    </div>
                    {m.installed ? (
                      <span className="text-emerald-400 font-medium">Installed</span>
                    ) : (
                      <button
                        type="button"
                        disabled={downloadingModel === m.id}
                        onClick={async () => {
                          if (!isTauri()) return;
                          setDownloadingModel(m.id);
                          try {
                            await invoke("download_whisper_model", { modelId: m.id });
                            const updated = await invoke<Array<{ id: string; size_label: string; installed: boolean; path: string }>>("list_whisper_models");
                            setWhisperModels(updated);
                          } catch (e) {
                            setConnStatus(`Whisper download error: ${String(e)}`);
                          } finally {
                            setDownloadingModel(null);
                          }
                        }}
                        className="px-2 py-0.5 rounded bg-amber-600/20 border border-amber-500/40 text-amber-200 hover:bg-amber-600/30 disabled:opacity-50"
                      >
                        {downloadingModel === m.id ? "Downloading..." : "Download"}
                      </button>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          <div className="p-3 rounded bg-zinc-950 border border-zinc-800 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Shield className="w-4 h-4 text-amber-400" />
              <div>
                <div className="font-semibold text-zinc-200">AI Safety Gate</div>
                <div className="text-[11px] text-zinc-500">
                  Require operator approval for go_live, clear, blackout, switch_output
                </div>
              </div>
            </div>
            <input
              type="checkbox"
              checked={requireApproval}
              onChange={(e) => setRequireApproval(e.target.checked)}
              className="accent-indigo-500 w-4 h-4"
            />
          </div>

          <div className="p-3 rounded bg-zinc-950 border border-zinc-800 space-y-2">
            <div className="flex items-center justify-between">
              <span className="flex items-center gap-1.5 font-semibold text-zinc-200">
                <Globe className="w-3.5 h-3.5 text-emerald-400" />
                Remote MCP Bridge ({mcpToolCount} tools active)
              </span>
              <button
                type="button"
                onClick={() => navigator.clipboard?.writeText(tunnelCmd)}
                className="flex items-center gap-1 px-2 py-0.5 rounded bg-zinc-800 text-[11px] text-zinc-300 hover:text-white"
              >
                <Copy className="w-3 h-3" /> Copy Tunnel
              </button>
            </div>
            <code className="block bg-zinc-900 px-2.5 py-1.5 rounded text-[11px] text-emerald-300 font-mono">
              {tunnelCmd}
            </code>
          </div>
        </div>

        <div className="px-4 py-3 border-t border-zinc-800 flex justify-end">
          <button
            onClick={handleSave}
            className="px-4 py-1.5 rounded bg-indigo-600 hover:bg-indigo-500 text-xs font-semibold text-white"
          >
            Save & Close
          </button>
        </div>
      </div>
    </div>
  );
}
