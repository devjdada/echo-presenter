# 09 — Technology Stack & Crates

---

## Core Stack

| Component | Technology | Version | Purpose |
|---|---|---|---|
| **Desktop Framework** | Tauri 2 | Stable | Native desktop shell, multi-window, IPC |
| **Backend Language** | Rust | 1.99+ | Production logic, all engines |
| **Frontend** | React + TypeScript | React 19 | Operator UI |
| **UI Library** | Tailwind CSS + shadcn/ui | Latest | Design system |
| **GPU Rendering** | wgpu | 30 | Audience output rendering (in companion process) |
| **2D Vector Rendering** | Vello | Latest | GPU-accelerated 2D paths, shapes, gradients |
| **Text Rendering** | cosmic-text | Latest | Unicode shaping, layout, font fallback |
| **Media Pipeline** | GStreamer (gstreamer-rs) | 0.23+ | Video/audio playback, capture |
| **Media Processing** | FFmpeg (ffmpeg-next) | Latest | Offline transcoding, format conversion |
| **NDI** | grafton-ndi | 2.0 | NDI 6 input/output with premultiplied RGBA alpha |
| **Database** | SQLite (rusqlite) | Latest | App data, Bible text |
| **Full-Text Search** | SQLite FTS5 | Built-in | Bible and content search |

> [!WARNING]
> `winit` is **not used**. It conflicts with Tauri 2's Tao windowing event loop. The companion `echo-renderer` process creates native OS windows directly via `windows-rs` (Win32) and attaches `wgpu` surfaces to them.

---

## Rust Crates (Critical Dependencies — 2026 Releases)

```toml
# === Core ===
tauri = "2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# === Graphics (in echo-renderer companion process) ===
wgpu = "30"
vello = "0.6"                 # GPU-accelerated 2D rendering
cosmic-text = "0.14"          # Text shaping + layout
image = "0.25"                # Image loading/processing
glam = "0.29"                 # Math (vectors, matrices)
# NOTE: NO winit — conflicts with Tauri. Use windows-rs for native windows.
windows = "0.61"              # Win32 API for window creation in renderer

# === Media ===
gstreamer = "0.23"
gstreamer-app = "0.23"
gstreamer-video = "0.23"
# ffmpeg-next = "7"           # Optional: offline transcoding

# === NDI ===
grafton-ndi = "2.0"           # NDI 6 with premultiplied RGBA alpha I/O

# === Audio ===
cpal = { version = "0.15", features = ["asio"] }   # Audio capture with ASIO support
rubato = "0.16"              # Resampling to 16kHz
whisper-rs = { version = "0.16", features = ["cuda", "vulkan"] }  # GPU-accelerated STT

# === Bible & Search ===
rusqlite = { version = "0.32", features = ["bundled"] }
aho-corasick = "1"           # Fast pattern matching
# ort = "2"                  # ONNX Runtime for embeddings (Phase 5)

# === Document Ingest ===
pdf-extract = "0.7"          # PDF text extraction
docx-rs = "0.5"              # DOCX parsing
# calamine = "0.26"          # XLSX (future)

# === AI ===
reqwest = { version = "0.12", features = ["json", "stream"] }  # HTTP client
eventsource-stream = "0.3"   # SSE parsing for streaming AI

# === MCP ===
rmcp = { version = "3.11", features = ["transport-io", "transport-streamable-http-server"] }

# === Hardware Integration ===
midir = "0.10"               # MIDI
rosc = "0.10"                # OSC
# artnet_protocol = "0.3"    # DMX/Art-Net (Phase 6)

# === Networking ===
axum = "0.8"                 # HTTP server for API
tokio-tungstenite = "0.24"   # WebSocket
tower = "0.5"                # Middleware
tower-http = "0.6"           # CORS, etc.

# === IPC (Tauri ↔ echo-renderer) ===
interprocess = "2"           # Named pipes (Windows) / Unix domain sockets
rmp-serde = "1"              # MessagePack for fast scene graph serialization

# === Utilities ===
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"
tracing = "0.1"              # Logging
tracing-subscriber = "0.3"
thiserror = "2"
anyhow = "1"
directories = "5"            # App data paths
notify = "7"                 # File watching
```

---

## Alternative Under Evaluation: `skia-safe`

If Decision 7 selects Skia over Vello + cosmic-text:

```toml
skia-safe = { version = "0.80", features = ["gpu", "textlayout"] }
# Replaces: vello, cosmic-text
# Trade-off: battle-tested text rendering, but C++ FFI + ~20 MB binary
```

---

## Frontend Dependencies

```json
{
  "dependencies": {
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-os": "^2",
    "@tauri-apps/plugin-fs": "^2",
    "@tauri-apps/plugin-dialog": "^2",
    "react": "^19",
    "react-dom": "^19",
    "typescript": "^5.5",
    "tailwindcss": "^4",
    "@radix-ui/react-*": "latest",
    "lucide-react": "latest",
    "zustand": "^5",
    "react-query": "^5",
    "framer-motion": "^11",
    "monaco-editor": "latest",
    "dnd-kit": "latest"
  }
}
```
