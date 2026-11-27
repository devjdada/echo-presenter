# 10 — Project Structure

---

## Directory Layout

```
echo-presenter/
│
├── apps/
│   ├── desktop/                        # Tauri 2 desktop application
│   │   ├── src-tauri/                  # Rust backend (main Tauri process)
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── commands/           # Tauri IPC commands
│   │   │   │   ├── state.rs            # App state management
│   │   │   │   └── setup.rs            # App initialization
│   │   │   ├── Cargo.toml
│   │   │   └── tauri.conf.json         # bundle id: com.echopresenter.app
│   │   └── src/                        # React frontend
│   │       ├── App.tsx
│   │       ├── pages/
│   │       │   ├── PresentationStudio.tsx
│   │       │   ├── VisualStudio.tsx
│   │       │   ├── BibleStudio.tsx
│   │       │   ├── AudioIntelligence.tsx
│   │       │   └── EchoAI.tsx
│   │       ├── components/
│   │       │   ├── layout/             # 4-zone operator layout
│   │       │   ├── slide-grid/         # Color-coded slide grid
│   │       │   ├── live-preview/       # Live preview + output toggles
│   │       │   ├── bottom-dock/        # Collapsible dock panels
│   │       │   └── common/
│   │       ├── hooks/
│   │       ├── stores/                 # Zustand stores
│   │       └── styles/
│   │
│   ├── renderer/                       # echo-renderer companion process
│   │   ├── src/
│   │   │   ├── main.rs                 # Standalone executable
│   │   │   ├── ipc.rs                  # Named pipe / UDS server
│   │   │   ├── scene_graph.rs          # Scene Graph JSON evaluator
│   │   │   ├── compositor.rs           # Layer blending, transitions
│   │   │   ├── text_renderer.rs        # Vello + cosmic-text
│   │   │   ├── output_manager.rs       # Multi-output with Looks
│   │   │   └── window.rs              # Native Win32 window creation
│   │   └── Cargo.toml
│   │
│   └── remote-control/                 # Future: browser companion app
│
├── crates/
│   ├── echo-core/                      # Shared types, traits, event bus, Scene Graph schema
│   ├── echo-presentation/              # Presentation engine
│   ├── echo-bible/                     # Bible engine
│   ├── echo-media/                     # GStreamer media pipeline
│   ├── echo-audio/                     # Audio capture & processing (WASAPI/ASIO)
│   ├── echo-transcription/             # Whisper STT (CPU + GPU tiers)
│   ├── echo-scripture-detection/       # Scripture detection pipeline
│   ├── echo-quotes/                    # Live Quotable Quotes extraction engine
│   ├── echo-ingest/                    # Document/Story-to-Slides pipeline
│   ├── echo-ai/                        # AI provider abstraction
│   ├── echo-automation/                # Macros, workflows, cue system
│   ├── echo-ndi/                       # NDI 6 integration (grafton-ndi)
│   ├── echo-api/                       # REST API server (Axum)
│   └── echo-mcp/                       # MCP server (rmcp 3.11)
│
├── resources/
│   ├── bible-modules/                  # Public domain Bible data
│   │   └── bundled_public_domain.sqlite  # KJV, ASV, WEB, YLT, DBY
│   ├── themes/                         # Built-in presentation themes
│   ├── templates/                      # Slide templates (Scene Graph JSON)
│   │   ├── title_hero.json
│   │   ├── scripture_split.json
│   │   ├── quote_card.json
│   │   ├── story_visual.json
│   │   └── two_column_point.json
│   ├── sample-media/                   # Demo backgrounds/images
│   └── ai-models/                      # Whisper models, embeddings
│       ├── ggml-base.en.bin
│       └── ggml-small.en.bin
│
├── tests/
│   ├── integration/
│   └── e2e/
│
├── docs/
│   ├── 00_INDEX.md
│   ├── 01_PRODUCT_VISION.md
│   ├── ...
│   └── 18_QUOTE_AND_CONTENT_PIPELINE.md
│
├── Cargo.toml                          # Workspace root
├── package.json
└── README.md
```

> [!NOTE]
> The `apps/renderer/` directory houses the **`echo-renderer` companion process** — a separate native executable that owns all GPU output windows. It communicates with the main Tauri process via named pipes (Windows) or Unix domain sockets, receiving Scene Graph JSON and rendering it via wgpu + Vello + cosmic-text.
